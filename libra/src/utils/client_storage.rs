use crate::command;
use crate::command::load_object;
use crate::internal::branch::Branch;
use crate::internal::head::Head;
use byteorder::{BigEndian, ReadBytesExt};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use lru_mem::LruCache;
use mercury::errors::GitError;
use mercury::hash::SHA1;
use mercury::internal::object::commit::Commit;
use mercury::internal::object::types::ObjectType;
use mercury::internal::pack::cache_object::CacheObject;
use mercury::internal::pack::Pack;
use mercury::utils::read_sha1;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashSet;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::{fs, io};
static PACK_OBJ_CACHE: Lazy<Mutex<LruCache<String, CacheObject>>> = Lazy::new(|| {
    // `lazy_static!` may affect IDE's code completion
    Mutex::new(LruCache::new(1024 * 1024 * 200))
});

#[derive(Default)]
///.libra/objects文件夹进行操作
pub struct ClientStorage {
    base_path: PathBuf,
}

impl ClientStorage {
    /// create `base_path` directory
    /// - `base_path` should be ".../objects"
    /// 一般是.libra/objects
    pub fn init(base_path: PathBuf) -> ClientStorage {
        fs::create_dir_all(&base_path).expect("Create directory failed!");
        ClientStorage { base_path }
    }

    /// e.g. 6ae8a755... -> 6a/e8a755...
    ///objectes文件夹下面，以40位hash字符串，前两位作为文件夹名，后38位作为文件名
    fn transform_path(&self, hash: &SHA1) -> String {
        let hash = hash.to_string();
        Path::new(&hash[0..2])
            .join(&hash[2..hash.len()])
            .into_os_string()
            .into_string()
            .unwrap()
    }

    /// join `base_path` and `obj_id` to get the full path of the object
    /// 给定hash值，返回对象完整的路径
    fn get_obj_path(&self, obj_id: &SHA1) -> PathBuf {
        Path::new(&self.base_path).join(self.transform_path(obj_id))
    }
    ///给定hash值，返回对象的ObjectType类型
    pub fn get_object_type(&self, obj_id: &SHA1) -> Result<ObjectType, GitError> {
        if self.exist_loosely(obj_id) {
            let raw_data = self.read_raw_data(obj_id)?;
            let data = Self::decompress_zlib(&raw_data)?;
            let (obj_type, _, _) = Self::parse_header(&data);
            ObjectType::from_string(&obj_type)
        } else {
            self.get_from_pack(obj_id)?
                .map(|x| x.1)
                .ok_or(GitError::ObjectNotFound(obj_id.to_string()))
        }
    }

    /// Check if the object with `obj_id` is of type `obj_type`
    /// 给定hash值，判断对象的类型是否为obj_type
    pub fn is_object_type(&self, obj_id: &SHA1, obj_type: ObjectType) -> bool {
        match self.get_object_type(obj_id) {
            Ok(t) => t == obj_type,
            Err(_) => false,
        }
    }

    /// Search objects that start with `obj_id`, loose & pack
    /// add support for relative path
    pub async fn search(&self, obj_id: &str) -> Vec<SHA1> {
        if obj_id == "HEAD" {
            return vec![Head::current_commit().await.unwrap()];
        }
        if obj_id.contains('~') || obj_id.contains('^') {
            // Find the position of the last non-~^ symbol and split into base reference and path.
            let mut split_pos = 0;
            let mut found_special = false;

            for (i, c) in obj_id.char_indices() {
                if c == '~' || c == '^' {
                    found_special = true;
                    split_pos = i;
                    break;
                }
            }

            if found_special {
                let base_ref = &obj_id[..split_pos];
                let path_part = &obj_id[split_pos..];

                let base_commit = match base_ref {
                    "HEAD" => Head::current_commit().await.unwrap(),
                    _ => {
                        if let Some(branch) = Branch::find_branch(base_ref, None).await {
                            branch.commit
                        } else {
                            let matches: Vec<SHA1> = self
                                .list_objects_pack()
                                .into_iter()
                                .chain(self.list_objects_loose().into_iter())
                                .filter(|x| self.is_object_type(x, ObjectType::Commit))
                                .filter(|x| x.to_string().starts_with(base_ref))
                                .collect();

                            if matches.len() == 1 {
                                matches[0]
                            } else {
                                return Vec::new();
                            }
                        }
                    }
                };
                let target_commit = match self.navigate_commit_path(base_commit, path_part) {
                    Ok(commit) => commit,
                    Err(_) => return Vec::new(),
                };

                return vec![target_commit];
            }
        }

        let mut objs = self.list_objects_pack();
        objs.extend(self.list_objects_loose());

        objs.into_iter()
            .filter(|x| x.to_string().starts_with(obj_id))
            .collect()
    }
    /// Navigates through commit history following a Git-style reference path
    /// For example: given "^2~3", navigate from base_commit to its second parent,
    /// then follow first parent three generations up
    ///
    /// Parameters:
    /// - base_commit: Starting commit SHA1
    /// - path: Reference path string (e.g. "^2~3", "~~~", "^~^2")
    ///
    fn navigate_commit_path(&self, base_commit: SHA1, path: &str) -> Result<SHA1, GitError> {
        let mut current = base_commit;

        let re = Regex::new(r"(\^|~)(\d*)").unwrap();

        if !re.is_match(path) {
            return Err(GitError::InvalidArgument(format!(
                "Invalid reference path: {}",
                path
            )));
        }
        for cap in re.captures_iter(path) {
            let symbol = cap.get(1).unwrap().as_str();
            let num_str = cap.get(2).map_or("1", |m| m.as_str());
            let num: usize = num_str.parse().unwrap_or(1);

            match symbol {
                "^" => {
                    current = self.get_parent_commit(&current, num)?;
                }
                "~" => {
                    for _ in 0..num {
                        current = self.get_parent_commit(&current, 1)?;
                    }
                }
                _ => unreachable!(),
            }
        }

        Ok(current)
    }
    /// get the reference of HEAD, e.g. HEAD~1, HEAD^2
    #[allow(dead_code)]
    async fn parse_head_reference(&self, reference: &str) -> Result<SHA1, GitError> {
        let mut current = Head::current_commit().await.unwrap();

        if reference == "HEAD" {
            return Ok(current);
        }

        let re = Regex::new(r"(\^|~)(\d*)").unwrap();
        let path = &reference[4..];
        if !re.is_match(path) {
            return Err(GitError::InvalidArgument(reference.to_string()));
        }

        for cap in re.captures_iter(path) {
            let symbol = cap.get(1).unwrap().as_str();
            let num_str = cap.get(2).map_or("1", |m| m.as_str());
            let num: usize = num_str.parse().unwrap_or(1);

            match symbol {
                "^" => {
                    current = self.get_parent_commit(&current, num)?;
                }
                "~" => {
                    for _ in 0..num {
                        current = self.get_parent_commit(&current, 1)?;
                    }
                }
                _ => unreachable!(),
            }
        }
        Ok(current)
    }

    /// get the nth parent commit of a commit
    fn get_parent_commit(&self, commit_id: &SHA1, n: usize) -> Result<SHA1, GitError> {
        let commit: Commit = load_object(commit_id)?;

        // the index starts from 0
        if n == 0 || n > commit.parent_commit_ids.len() {
            return Err(GitError::ObjectNotFound(format!(
                "Parent {} does not exist",
                n
            )));
        }

        Ok(commit.parent_commit_ids[n - 1])
    }

    /// 解析 HEAD 相关的引用，如 HEAD, HEAD^, HEAD~3, HEAD^2~3 等
    #[allow(dead_code)]
    async fn parse_head_reference_old(&self, reference: &str) -> Result<SHA1, GitError> {
        let mut current = Head::current_commit().await.unwrap();

        // 如果只是 "HEAD"，直接返回
        if reference == "HEAD" {
            return Ok(current);
        }

        // 解析剩余的路径表达式
        let path = &reference[4..]; // 跳过 "HEAD" 四个字符
        let mut chars = path.chars();
        if reference == "HEAD^10" {
            println!("reference is {:?}", reference);
        }
        while let Some(c) = chars.next() {
            match c {
                '^' => {
                    // 处理 ^ 语法，如 HEAD^ 或 HEAD^2 或 HEAD^10
                    let mut parent_num = 1; // 默认为第一个父提交

                    // 检查下一个字符是否是数字
                    let mut is_digit = false;

                    // 收集所有连续的数字字符
                    while let Some(num_char) = chars.next() {
                        if num_char.is_ascii_digit() {
                            is_digit = true;
                            parent_num = parent_num * 10 + num_char.to_digit(10).unwrap() as usize;
                        } else {
                            // 不是数字，回退一个字符，结束数字解析
                            // 注意：这里原本的 chars.next() 是错误的，应该用 chars.next_back()
                            // 但 Chars 迭代器不支持 next_back()，所以我们需要一个不同的方案
                            break;
                        }
                    }

                    // 获取指定的父提交
                    current = self.get_parent_commit(&current, parent_num)?;
                }
                '~' => {
                    // 处理 ~ 语法，如 HEAD~3
                    let mut num_steps = 0;
                    while let Some(num_char) = chars.next() {
                        if num_char.is_ascii_digit() {
                            num_steps = num_steps * 10 + num_char.to_digit(10).unwrap() as usize;
                        } else {
                            chars.next(); // 回退一个字符
                            break;
                        }
                    }

                    if num_steps == 0 {
                        num_steps = 1; // 默认为1步
                    }

                    // 遍历 n 个第一父提交
                    for _ in 0..num_steps {
                        current = self.get_parent_commit(&current, 1)?;
                    }
                }
                _ => return Err(GitError::InvalidArgument("not a valid char".into())), // 处理无效的字符
            }
        }

        Ok(current)
    }

    /// list all objects' hash in `objects`
    /// 获取objects文件夹下所有对象的hash值，不包括info和pack文件夹
    fn list_objects_loose(&self) -> Vec<SHA1> {
        let mut objects = Vec::new();
        let paths = fs::read_dir(&self.base_path).unwrap();
        for path in paths {
            let path = path.unwrap().path();
            if path.is_dir() && path.file_name().unwrap().len() == 2 {
                // not very elegant
                let sub_paths = fs::read_dir(&path).unwrap();
                for sub_path in sub_paths {
                    let sub_path = sub_path.unwrap().path();
                    if sub_path.is_file() {
                        let parent_name = path.file_name().unwrap().to_str().unwrap().to_string();
                        let file_name = sub_path.file_name().unwrap().to_str().unwrap().to_string();
                        let file_name = parent_name + &file_name;
                        objects.push(SHA1::from_str(&file_name).unwrap()); // this will check format, so don't worry
                    }
                }
            }
        }
        objects
    }

    /// List all objects' hash in PACKs
    fn list_objects_pack(&self) -> HashSet<SHA1> {
        let idxes = self.list_all_idx();
        let mut objs = HashSet::new();
        for idx in idxes {
            let res = Self::list_idx_objects(&idx).unwrap();
            for obj in res {
                objs.insert(obj);
            }
        }
        objs
    }
}

impl ClientStorage {
    /// zlib header: 78 9C, but Git is 78 01
    fn compress_zlib(data: &[u8]) -> io::Result<Vec<u8>> {
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(data)?;
        let compressed_data = encoder.finish()?;
        Ok(compressed_data)
    }

    fn decompress_zlib(data: &[u8]) -> io::Result<Vec<u8>> {
        let mut decoder = ZlibDecoder::new(data);
        let mut decompressed_data = Vec::new();
        decoder.read_to_end(&mut decompressed_data)?;
        Ok(decompressed_data)
    }

    ///对象的写入方式都是 (类型_str +空格 + 数据长度_str)Head + '\0' + 数据_bytes
    fn parse_header(data: &[u8]) -> (String, usize, usize) {
        let end_of_header = data
            .iter()
            .position(|&b| b == b'\0')
            .expect("Invalid object: no header terminator");
        let header_str =
            std::str::from_utf8(&data[..end_of_header]).expect("Invalid UTF-8 in header");

        let mut parts = header_str.splitn(2, ' ');
        let obj_type = parts.next().expect("No object type in header").to_string();
        let size_str = parts.next().expect("No size in header");
        let size = size_str.parse::<usize>().expect("Invalid size in header");
        assert_eq!(size, data.len() - 1 - end_of_header, "Invalid object size");
        (obj_type, size, end_of_header)
    }
    ///给定hash值，读取对象的原始数据
    fn read_raw_data(&self, obj_id: &SHA1) -> Result<Vec<u8>, io::Error> {
        let path = self.get_obj_path(obj_id);
        let mut file = fs::File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    ///给定hash值，返回对象解压后的数据
    pub fn get(&self, object_id: &SHA1) -> Result<Vec<u8>, GitError> {
        if self.exist_loosely(object_id) {
            let raw_data = self.read_raw_data(object_id)?;
            let data = Self::decompress_zlib(&raw_data)?;

            // skip & check header
            let (_, _, end_of_header) = Self::parse_header(&data);
            Ok(data[end_of_header + 1..].to_vec())
        } else {
            // Ok(self.get_from_pack(object_id)?.unwrap().0)
            self.get_from_pack(object_id)?
                .map(|x| x.0)
                .ok_or(GitError::ObjectNotFound(object_id.to_string()))
        }
    }

    /// Save content to `objects`
    /// 传入hash值，作为保存路径
    /// 然后就是 类型 + 空格 + 数据长度 + '\0' + 数据，最后再压缩，然后写入数据 
    pub fn put(
        &self,
        obj_id: &SHA1,
        content: &[u8],
        obj_type: ObjectType,
    ) -> Result<String, io::Error> {
        let path = self.get_obj_path(obj_id);
        let dir = path.parent().unwrap();
        fs::create_dir_all(dir)?;

        let header = format!("{} {}\0", obj_type, content.len());
        let full_content = [header.as_bytes().to_vec(), Vec::from(content)].concat();

        let mut file = fs::File::create(&path)?;
        file.write_all(&Self::compress_zlib(&full_content)?)?;
        Ok(path.to_str().unwrap().to_string())
    }

    /// Check if the object with `obj_id` exists in `objects` or PACKs
    pub fn exist(&self, obj_id: &SHA1) -> bool {
        let path = self.get_obj_path(obj_id);
        Path::exists(&path) || self.get_from_pack(obj_id).unwrap().is_some()
    }

    /// Check if the object with `obj_id` exists in `objects`
    /// 给定hash值，判断有没有那个对象文件
    fn exist_loosely(&self, obj_id: &SHA1) -> bool {
        let path = self.get_obj_path(obj_id);
        Path::exists(&path)
    }
}
const FANOUT: u64 = 256 * 4;
// TODO refactor to `PackReader`
impl ClientStorage {
    /// List all .pack files in `pack` directory
    /// 获取.libra/objects/pack目录下所有的.pack文件
    fn list_all_packs(&self) -> Vec<PathBuf> {
        let pack_dir = self.base_path.join("pack");
        if !pack_dir.exists() {
            return Vec::new();
        }
        let mut packs = Vec::new();
        for entry in fs::read_dir(pack_dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_file() && path.extension().unwrap() == "pack" {
                packs.push(path);
            }
        }
        packs
    }

    /// List all .idx files in `pack` directory
    /// - If .idx file not exists, build it
    /// 每个.pack文件也对应一个idx文件，返回所有.idx文件的路径，实际还是在pack文件夹
    fn list_all_idx(&self) -> Vec<PathBuf> {
        let packs = self.list_all_packs();
        let mut idxs = Vec::new();
        for pack in packs {
            let idx = pack.with_extension("idx");
            if !idx.exists() {
                command::index_pack::build_index_v1(pack.to_str().unwrap(), idx.to_str().unwrap())
                    .unwrap();
            }
            idxs.push(idx);
        }
        idxs
    }

    /// Get object from PACKs by hash, if not found, return None
    fn get_from_pack(&self, obj_id: &SHA1) -> Result<Option<(Vec<u8>, ObjectType)>, GitError> {
        let idxes = self.list_all_idx(); // list or build
        for idx in idxes {
            let res = Self::read_pack_by_idx(&idx, obj_id)?;
            if let Some(data) = res {
                return Ok(Some((data.data_decompressed.clone(), data.object_type())));
            }
        }

        Ok(None)
    }
    /*
    GitHub Copilot
    read_idx_fanout 函数解析
    read_idx_fanout 函数是 Git 索引文件（.idx）解析的核心部分，它读取 Git 包索引文件的扇出表（fanout table）。这个函数的作用如下：

    功能说明
    该函数从 Git 索引文件（.idx）中读取扇出表，这是一个包含 256 个条目的数组，用于加速对象查找。每个条目是一个 4 字节的无符号整数（u32），按大端字节序存储。

    实现细节
    打开索引文件：函数接收一个索引文件路径参数，并打开该文件进行读取。

    准备数据结构：创建一个 256 个元素的 u32 数组来存储扇出表条目。

    读取数据：使用迭代器遍历数组中的每个元素位置，每次：

    读取 4 字节到缓冲区
    将这 4 字节解析为大端序的 u32 值
    将该值存入数组对应位置
    返回结果：返回填充好的扇出表数组

    扇出表的作用
    扇出表是 Git 存储中的一种优化结构，它允许 Git 快速定位特定对象：

    表中的每个项表示特定对象 SHA-1 哈希前缀（第一个字节）的累积计数
    例如，索引中第 0x61 ('a') 位置的值表示哈希值第一个字节小于或等于 'a' 的对象数量
    这使得 Git 可以根据对象哈希的第一个字节，快速确定需要在索引中搜索的范围，而不是线性扫描整个索引
    这种扇出表结构极大提高了对象查找的效率，尤其是在存储库包含大量对象时。

    代码中的应用
    在后续的 read_idx 和 list_idx_objects 函数中，通过利用这个扇出表，可以快速：

    找到特定哈希值对象在 pack 文件中的偏移量位置
    列出所有包含在这个索引文件中的对象
    这是 Git 存储系统高效性的关键部分之一。

     */
    fn read_idx_fanout(idx_file: &Path) -> Result<[u32; 256], io::Error> {
        let mut idx_file = fs::File::open(idx_file)?;
        // const FANOUT: usize = 256 * 4;
        let mut fanout: [u32; 256] = [0; 256]; // 256 * 4 bytes
        let mut buf = [0; 4];
        fanout.iter_mut().for_each(|x| {
            idx_file.read_exact(&mut buf).unwrap();
            *x = u32::from_be_bytes(buf);
        });
        Ok(fanout)
    }

    /// List all objects hash in .idx file
    fn list_idx_objects(idx_file: &Path) -> Result<Vec<SHA1>, io::Error> {
        let fanout: [u32; 256] = Self::read_idx_fanout(idx_file)?; // TODO param change to `&mut File`, to auto seek
        let mut idx_file = fs::File::open(idx_file)?;
        idx_file.seek(io::SeekFrom::Start(FANOUT))?; // important!

        let mut objs = Vec::new();
        for _ in 0..fanout[255] {
            let _offset = idx_file.read_u32::<BigEndian>()?;
            let hash = read_sha1(&mut idx_file)?;

            objs.push(hash);
        }
        Ok(objs)
    }

    /// Read object `offset` from .idx file by `hash`
    fn read_idx(idx_file: &Path, obj_id: &SHA1) -> Result<Option<u64>, io::Error> {
        let fanout: [u32; 256] = Self::read_idx_fanout(idx_file)?;
        let mut idx_file = fs::File::open(idx_file)?;

        let first_byte = obj_id.0[0];
        let start = if first_byte == 0 {
            0
        } else {
            fanout[first_byte as usize - 1] as usize
        };
        let end = fanout[first_byte as usize] as usize;

        idx_file.seek(io::SeekFrom::Start(FANOUT + 24 * start as u64))?;
        for _ in start..end {
            let offset = idx_file.read_u32::<BigEndian>()?;
            let hash = read_sha1(&mut idx_file)?;

            if &hash == obj_id {
                return Ok(Some(offset as u64));
            }
        }

        Ok(None)
    }

    /// Get object from pack by .idx file
    fn read_pack_by_idx(idx_file: &Path, obj_id: &SHA1) -> Result<Option<CacheObject>, GitError> {
        let pack_file = idx_file.with_extension("pack");
        let res = Self::read_idx(idx_file, obj_id)?;
        match res {
            None => Ok(None),
            Some(offset) => {
                let res = Self::read_pack_obj(&pack_file, offset)?;
                Ok(Some(res))
            }
        }
    }

    /// Read object from pack file, with offset
    /// LRU缓存是 文件名-偏移作为key进行查找？
    fn read_pack_obj(pack_file: &Path, offset: u64) -> Result<CacheObject, GitError> {
        let cache_key = format!("{:?}-{}", pack_file.file_name().unwrap(), offset);
        // read cache
        if let Some(cached) = PACK_OBJ_CACHE.lock().unwrap().get(&cache_key) {
            return Ok(cached.clone());
        }

        let file = fs::File::open(pack_file)?;
        let mut pack_reader = io::BufReader::new(&file);
        pack_reader.seek(io::SeekFrom::Start(offset))?;
        let mut pack = Pack::new(None, None, None, false);
        //从文件里面读取缓存？
        let obj = {
            let mut offset = offset as usize;
            pack.decode_pack_object(&mut pack_reader, &mut offset)? // offset will be updated!
        };
        let full_obj = match obj.object_type() {
            ObjectType::OffsetDelta => {
                let base_offset = obj.offset_delta().unwrap();
                let base_obj = Self::read_pack_obj(pack_file, base_offset as u64)?;
                let base_obj = Arc::new(base_obj);
                Pack::rebuild_delta(obj, base_obj) // new obj
            }
            ObjectType::HashDelta => {
                let base_hash = obj.hash_delta().unwrap();
                let idx_file = pack_file.with_extension("idx");
                let base_offset = Self::read_idx(&idx_file, &base_hash)?.unwrap();
                let base_obj = Self::read_pack_obj(pack_file, base_offset)?;
                let base_obj = Arc::new(base_obj);
                Pack::rebuild_delta(obj, base_obj) // new obj
            }
            _ => obj,
        };
        // write cache
        if PACK_OBJ_CACHE
            .lock()
            .unwrap()
            .insert(cache_key, full_obj.clone())
            .is_err()
        {
            eprintln!("Warn: EntryTooLarge");
        }
        Ok(full_obj)
    }
}

#[cfg(test)]
mod tests {
    use mercury::internal::object::blob::Blob;
    use mercury::internal::object::types::ObjectType;
    use mercury::internal::object::ObjectTrait;
    use serial_test::serial;
    use std::fs;
    use std::path::PathBuf;

    use crate::utils::test;

    use super::ClientStorage;

    #[test]
    #[ignore]
    fn test_content_store() {
        let content = "Hello, world!";
        let blob = Blob::from_content(content);

        let mut source = PathBuf::from(test::find_cargo_dir().parent().unwrap());
        source.push("tests/objects");

        let client_storage = ClientStorage::init(source.clone());
        assert!(client_storage
            .put(&blob.id, &blob.data, blob.get_type())
            .is_ok());
        assert!(client_storage.exist(&blob.id));

        let data = client_storage.get(&blob.id).unwrap();
        assert_eq!(data, blob.data);
        assert_eq!(String::from_utf8(data).unwrap(), content);
    }

    #[tokio::test]
    /// Tests object search functionality by partial hash prefix.
    /// Verifies that objects can be correctly found when searching with a partial SHA1 hash.
    async fn test_search() {
        let blob = Blob::from_content("Hello, world!");

        let mut source = PathBuf::from(test::find_cargo_dir().parent().unwrap());
        source.push("tests/objects");

        let client_storage = ClientStorage::init(source.clone());
        assert!(client_storage
            .put(&blob.id, &blob.data, blob.get_type())
            .is_ok());

        let objs = client_storage.search("5dd01c177").await;

        assert_eq!(objs.len(), 1);
    }

    #[test]
    #[serial]
    /// Prints all object hashes found in the test objects directory.
    fn test_list_objs() {
        let mut source = PathBuf::from(test::find_cargo_dir().parent().unwrap());
        source.push("tests/objects");
        let client_storage = ClientStorage::init(source);
        let objs = client_storage.list_objects_loose();
        for obj in objs {
            println!("{}", obj);
        }
    }

    #[test]
    #[serial]
    #[ignore]
    ///tests the function of get_object_type can get the object's type right.
    fn test_get_obj_type() {
        let blob = Blob::from_content("Hello, world!");

        let mut source = PathBuf::from(test::find_cargo_dir().parent().unwrap());
        source.push("tests/objects");

        let client_storage = ClientStorage::init(source.clone());
        assert!(client_storage
            .put(&blob.id, &blob.data, blob.get_type())
            .is_ok());

        let obj_type = client_storage.get_object_type(&blob.id).unwrap();
        assert_eq!(obj_type, ObjectType::Blob);
    }

    #[test]
    ///Tests confirm that the compression and decompression features are functioning as expected.
    fn test_decompress() {
        let data = b"blob 13\0Hello, world!";
        let compressed_data = ClientStorage::compress_zlib(data).unwrap();
        let decompressed_data = ClientStorage::decompress_zlib(&compressed_data).unwrap();
        assert_eq!(decompressed_data, data);
    }

    #[test]
    #[serial]
    /// Tests decompression of a specific git object file from test data to verify zlib implementation.
    fn test_decompress_2() {
        test::reset_working_dir();
        let pack_file = "../tests/data/objects/4b/00093bee9b3ef5afc5f8e3645dc39cfa2f49aa";
        let pack_content = fs::read(pack_file).unwrap();
        let decompressed_data = ClientStorage::decompress_zlib(&pack_content).unwrap();
        println!("{:?}", String::from_utf8(decompressed_data).unwrap());
    }
}
