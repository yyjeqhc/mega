use crate::utils::util;
use std::path::PathBuf;

///.libra/index文件夹
pub fn index() -> PathBuf {
    util::storage_path().join("index")
}
///.libra/objects文件夹
pub fn objects() -> PathBuf {
    util::storage_path().join("objects")
}
///.libra/libra.db文件
pub fn database() -> PathBuf {
    util::storage_path().join(util::DATABASE)
}
///.libra/.libra_attributes文件
pub fn attributes() -> PathBuf {
    util::working_dir().join(util::ATTRIBUTES)
}
