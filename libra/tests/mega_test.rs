use http::Method;
use lazy_static::lazy_static;
use libra::internal::protocol::lfs_client::LFSClient;
use libra::internal::protocol::ProtocolClient;
use libra::utils::lfs;
use reqwest::Url;
use std::env;
use std::net::TcpStream;
use std::path::PathBuf;
use testcontainers::core::wait::HttpWaitStrategy;
use testcontainers::{
    core::{IntoContainerPort, Mount, ReuseDirective, WaitFor},
    runners::AsyncRunner,
    ContainerAsync, GenericImage, ImageExt,
};
use tokio::time::Duration;

lazy_static! {

    static ref TARGET: String = {
        // mega/mega, absolute
        let mut manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR")); // Get env at compile time
        manifest.pop();
        manifest.to_str().unwrap().to_string()
    };

    static ref LIBRA: PathBuf = {
        let path = if cfg!(target_os = "windows") {
            format!("{}/target/debug/libra.exe", TARGET.as_str())
        } else {
            format!("{}/target/debug/libra", TARGET.as_str())
        };
        PathBuf::from(path)
    };

    static ref MEGA: PathBuf = {
        let path = if cfg!(target_os = "windows") {
            format!("{}/target/debug/mega.exe", TARGET.as_str())
        } else {
            format!("{}/target/debug/mega", TARGET.as_str())
        };
        PathBuf::from(path)
    };

    static ref CONFIG: PathBuf = {
        let path =  format!("{}/mega/config.toml",TARGET.as_str());
        PathBuf::from(path)
    };
}

fn is_port_in_use(port: u16) -> bool {
    TcpStream::connect_timeout(
        &format!("127.0.0.1:{}", port).parse().unwrap(),
        Duration::from_millis(1000),
    )
    .is_ok()
}

async fn mega_container(mapping_port: u16) -> ContainerAsync<GenericImage> {
    println!("MEGA {:?} ", MEGA.to_str().unwrap());
    println!("CONFIG {:?} ", CONFIG.to_str().unwrap());
    if !MEGA.exists() {
        panic!("mega binary not found in \"target/debug/\", skip lfs test");
    }
    if is_port_in_use(mapping_port) {
        panic!("port {} is already in use", mapping_port);
    }
    let port_str = mapping_port.to_string();
    let cmd = vec![
        "/root/mega",
        "service",
        "multi",
        "http",
        "-p",
        &port_str,
        "--host",
        "0.0.0.0",
    ];

    GenericImage::new("ubuntu", "latest")
        .with_exposed_port(mapping_port.tcp())
        .with_wait_for(WaitFor::Http(Box::new(
            HttpWaitStrategy::new("/")
                .with_method(Method::GET)
                .with_expected_status_code(404_u16),
        )))
        .with_mapped_port(mapping_port, mapping_port.tcp())
        .with_mount(Mount::bind_mount(MEGA.to_str().unwrap(), "/root/mega"))
        .with_mount(Mount::bind_mount(
            CONFIG.to_str().unwrap(),
            "/root/config.toml",
        ))
        .with_env_var("MEGA_authentication__enable_http_auth", "false")
        .with_working_dir("/root")
        .with_reuse(ReuseDirective::Never)
        .with_cmd(cmd)
        .start()
        .await
        .expect("Failed to start mega_server")
}

pub async fn mega_bootstrap_servers(mapping_port: u16) -> (ContainerAsync<GenericImage>, String) {
    let container = mega_container(mapping_port).await;
    let mega_ip = container.get_bridge_ip_address().await.unwrap();
    let mega_port: u16 = container.get_host_port_ipv4(mapping_port).await.unwrap();
    (container, format!("http://{}:{}", mega_ip, mega_port))
}

#[tokio::test]
#[ignore]
///Use container to run mega server and test push and download
async fn test_push_object_and_download() {
    let (_container, mega_server_url) = mega_bootstrap_servers(12000).await;
    println!("container: {}", mega_server_url);
    let file_map = mercury::test_utils::setup_lfs_file().await;
    let file = file_map
        .get("git-2d187177923cd618a75da6c6db45bb89d92bd504.pack")
        .unwrap();
    let client = LFSClient::from_url(&Url::parse(&mega_server_url).unwrap());
    let oid = lfs::calc_lfs_file_hash(file).unwrap();

    match client.push_object(&oid, file).await {
        Ok(_) => println!("Pushed successfully."),
        Err(err) => eprintln!("Push failed: {:?}", err),
    }
    #[cfg(feature = "p2p")]
    test_download_chunk_mega(&mega_server_url, &oid).await;
}

#[cfg(feature = "p2p")]
async fn test_download_chunk_mega(mega_server_url: &str, oid: &str) {
    // let file_map = mercury::test_utils::setup_lfs_file().await;
    // let file = file_map
    //     .get("git-2d187177923cd618a75da6c6db45bb89d92bd504.pack")
    //     .unwrap();
    let client = LFSClient::from_url(&Url::parse(mega_server_url).unwrap());
    // let oid = lfs::calc_lfs_file_hash(file).unwrap();
    let sub_oid = "ee225720cc31599c749fbe9b18f6c8346fa3246839f0dea7ffd3224dbb067952".to_string(); // offset 83886080 size 20971520
    let url = format!("{}/objects/{}/{}", mega_server_url, oid, sub_oid);
    let size = 20971520;
    let offset = 83886080;
    let data = client
        .download_chunk(&url, &sub_oid, size, offset, |_| {})
        .await
        .unwrap();
    println!(
        "test_download_chunk_mega success. download_len {}",
        data.len()
    );
    assert_eq!(data.len(), size);
}
use reqwest::Client;
use std::fs::File;
use std::io::Seek;
use std::io::Write;
use tokio::task;
#[tokio::test]
#[cfg(feature = "p2p")]
async fn test_download() {
    // let mega_server_url = "http://localhost:58001";
    // let client = LFSClient::from_url(&Url::parse(mega_server_url).unwrap());
    // let sub_oid = "e6ae2f064fee533bfd85a5320355dac4ed596522c38b480ed8accf09f12ade2d";
    // let oid = "322bb6428a1d20aded70180a19bbe1ba1bec80500bb7dac224abd7da7a855d9e".to_string(); // offset 83886080 size 20971520
    // let url = format!("{}/objects/{}/{}", mega_server_url, oid, sub_oid);
    // let size = 17962477;
    // let offset = 62914560;
    // let data = client
    //     .download_chunk(&url, &sub_oid, size, offset, |_| {})
    //     .await
    //     .unwrap();
    // println!(
    //     "test_download_chunk_mega success. download_len {}",
    //     data.len()
    // );
    // println!("url {:?}",url);
    // assert_eq!(data.len(), size);
    let mega_server_url = "http://localhost:58001";
    let oid = "322bb6428a1d20aded70180a19bbe1ba1bec80500bb7dac224abd7da7a855d9e";

    // 分片信息
    let chunks = vec![
        (
            "adb927c0074d357ce49ee28c91ba496682a5ca4604ce7c66d6c33f39e4f65b93",
            0,
            20971520,
        ),
        (
            "a04d0f05a4d2094be0ab40949f694150c8301283963670e19b1db7c517ec369d",
            20971520,
            20971520,
        ),
        (
            "b3f54418917b1da93b21bb00325ff2698e3ab88052a7a94d1722ab63884912ac",
            41943040,
            20971520,
        ),
        (
            "e6ae2f064fee533bfd85a5320355dac4ed596522c38b480ed8accf09f12ade2d",
            62914560,
            17962477,
        ),
    ];

    // 输出文件
    let output_path = "/root/clone/a.flac";
    let mut file = File::create(&output_path).unwrap();

    // 创建 HTTP 客户端
    let client = LFSClient::from_url(&Url::parse(mega_server_url).unwrap());

    // 按 offset 顺序下载并写入分片
    for (sub_oid, offset, size) in chunks {
        let url = format!("{}/objects/{}/{}", mega_server_url, oid, sub_oid);
        println!(
            "Downloading chunk: url={}, sub_oid={}, offset={}, size={}",
            url, sub_oid, offset, size
        );

        let data = client
            .download_chunk(&url, sub_oid, size, offset, |_| {})
            .await
            .unwrap();
        if data.len() != size {
            println!(
                "Chunk {} size mismatch: expected {}, got {}",
                sub_oid,
                size,
                data.len()
            );
        }

        // 写入文件（按 offset）
        file.seek(std::io::SeekFrom::Start(offset as u64)).unwrap();
        file.write_all(&data).unwrap();
        println!(
            "Chunk {} downloaded and written successfully, size={}",
            sub_oid,
            data.len()
        );
    }

    // 确保文件刷新到磁盘
    file.flush().unwrap();
    println!("File assembled successfully: {}", output_path);
}

#[tokio::test]
async fn test_download_object() {
    println!("current_dir {:?}", env::current_dir());
    let oid = "322bb6428a1d20aded70180a19bbe1ba1bec80500bb7dac224abd7da7a855d9e";
    let lfs_obj_path = "/root/clone/";
    let path_abs = lfs::lfs_object_path(oid);
    if false {
        // found in local cache
        println!("already have file.");
    } else {
        let size = 80877037;
        // not exist, download from server
        if let Err(e) = LFSClient::get()
            .await
            .download_object(&oid, size, &path_abs, None)
            .await
        {
            eprintln!("fatal: {}", e);
        }
    }
}
#[test]
fn test_url() {
    let url = "http://localhost:58001/third-part/lfs.git";
    let u = Url::parse(&url).unwrap();
    println!("{:?}", u);
}
