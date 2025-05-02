use std::path::PathBuf;
use std::sync::Arc;

use crate::fuse::MegaFuse;
use crate::manager::fetch::fetch;
use crate::manager::{ScorpioManager, WorkDir};
use crate::util::{config, GPath};
use axum::extract::State;
use axum::routing::{get, post};
use axum::Router;
use mercury::hash::SHA1;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
mod git;
const SUCCESS: &str = "Success";
const FAIL: &str = "Fail";

#[derive(Debug, Deserialize, Serialize)]
struct MountRequest {
    path: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct MountResponse {
    status: String,
    mount: MountInfo,
    message: String,
}
#[derive(Debug, Deserialize, Serialize, Default)]
struct MountInfo {
    hash: String,
    path: String,
    inode: u64,
}

#[derive(Debug, Deserialize, Serialize)]
struct MountsResponse {
    status: String,
    mounts: Vec<MountInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
struct UmountRequest {
    path: Option<String>,
    inode: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize)]
struct UmountResponse {
    status: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ConfigResponse {
    status: String,
    config: ConfigInfo,
}

#[derive(Debug, Deserialize, Serialize)]
struct ConfigInfo {
    mega_url: String,
    mount_path: String,
    store_path: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ConfigRequest {
    mega_url: Option<String>,
    mount_path: Option<String>,
    store_path: Option<String>,
}
#[derive(Clone)]
struct ScoState {
    fuse: Arc<MegaFuse>,
    manager: Arc<Mutex<ScorpioManager>>,
}
#[allow(unused)]
pub async fn daemon_main(fuse: Arc<MegaFuse>, manager: ScorpioManager) {
    let inner = ScoState {
        fuse,
        manager: Arc::new(Mutex::new(manager)),
    };
    let ml_watch = inner.manager.clone();
    let mut app = Router::new()
        .route("/api/fs/mount", post(mount_handler))
        .route("/api/fs/mpoint", get(mounts_handler))
        .route("/api/fs/umount", post(umount_handler))
        .route("/api/config", get(config_handler))
        .route("/api/config", post(update_config_handler))
        .route("/api/git/status", get(git::git_status_handler))
        .route("/api/git/commit", post(git::git_commit_handler))
        .route("/api/git/push", post(git::git_push_handler))
        .route("/api/git/add", post(git::git_add_handler))
        .route("/api/git/reset", post(git::git_reset_handler))
        .with_state(inner.clone());

    // LFS route & merge it
    let lfs_route = crate::scolfs::route::router();
    let app = app.merge(lfs_route);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:2725").await.unwrap();
    // tokio::spawn(watch_temp(inner));

    axum::serve(listener, app).await.unwrap();
    // tokio::spawn(watch_dir(ml_watch));
}
async fn watch_temp(state: ScoState) {
    tokio::time::sleep(tokio::time::Duration::from_secs(20)).await;
    let megafuse = state.fuse.clone();
    let store = megafuse.dic.store.clone();
    println!(
        "{:?}",
        store
            .get_inode_from_path("/third-party/stw/1.txt")
            .await
            .unwrap()
    );
    println!(
        "{:?}",
        store
            .get_inode_from_path("/third-party/world/t")
            .await
            .unwrap()
    );

    // loop {
    //     {

    //     }
    //     tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    // }
}
async fn my_update(state: ScoState) {
    // let megafuse = state.fuse.clone();
    // let store_path = config::store_path();
    // let mag = state.manager.clone();
    // loop {
    //     {
    //         let manager = &*mag.lock().await;

    //         for dir in &manager.works {
    //             let _lower = PathBuf::from(store_path).join(&dir.hash);
    //             if true {
    //                 let path = dir.path.to_owned();
    //                 let p = GPath::from(path);

    //                 let tree = fetch_tree(&p).await.unwrap();
    //                 //判断树有没有更新，没有更新，就不管，还有就是锁的问题
    //                 println!("my_update fetch get the tree {:?}", tree.id.to_string());

    //                 let work_path = PathBuf::from(store_path).join(&dir.hash);
    //                 let _lower = work_path.join("lower");
    //                 fetch_code(&p, _lower).await.unwrap();

    //                 set_parent_commit(&work_path).await.unwrap();
    //             }
    //             megafuse.overlay_mount(dir.node, &_lower).await.unwrap();
    //         }
    //     }
    //     println!("完成一轮更新，等待60秒后继续...");
    //     tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    // }
    let megafuse = state.fuse.clone();
    let store_path = config::store_path();
    loop {
        {
            let manager = &*state.manager.lock().await;

            for dir in &manager.works {
                let lower = PathBuf::from(store_path).join(&dir.hash);

                let path = dir.path.to_owned();
                let p = GPath::from(path);
                let _ = fetch_tree(&p).await.unwrap();
                let work_path = lower.join("lower");
                fetch_code(&p, work_path).await.unwrap();
                set_parent_commit(&lower).await.unwrap();

                megafuse.overlay_mount(dir.node, &lower).await.unwrap();
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}
use crate::manager::fetch::fetch_code;
use crate::manager::fetch::fetch_tree;
use crate::manager::fetch::set_parent_commit;
async fn watch_dir(manager: Arc<Mutex<ScorpioManager>>) {
    // 创建无限循环，实现持续监控
    loop {
        println!("开始监控工作目录并更新...");

        // 对每个工作路径进行处理
        {
            let mut ml = manager.lock().await;

            // 获取当前所有工作目录的副本，避免持有锁时间过长
            let works_copy = ml.works.clone();

            // 释放锁，以便其他操作可以进行
            drop(ml);

            // 遍历所有工作目录
            for work_dir in &works_copy {
                println!("正在更新工作目录: {}", work_dir.path);

                // 重新获取锁
                let mut ml = manager.lock().await;

                // 获取工作目录的inode和路径
                let inode = work_dir.node;
                let mono_path = work_dir.path.clone();

                // 使用GPath规范化路径
                let p = GPath::from(mono_path.clone());

                // 获取树和哈希值
                match fetch_tree(&p).await {
                    Ok(tree) => {
                        // 创建WorkDir结构体表示工作目录
                        let workdir = WorkDir {
                            path: p.to_string(),
                            node: inode,
                            hash: tree.id.to_string(),
                        };
                        println!(
                            "{:?} {:?} {:?}",
                            work_dir.hash, work_dir.path, work_dir.node
                        );
                        // 构建存储路径
                        let store_path = config::store_path();
                        let work_path = PathBuf::from(store_path).join(&workdir.hash);
                        let lower = work_path.join("lower");

                        // 只有当哈希值发生变化时才更新代码
                        if work_dir.hash != tree.id.to_string() {
                            println!(
                                "检测到更新: {} 哈希从 {} 变为 {}",
                                mono_path,
                                work_dir.hash,
                                tree.id.to_string()
                            );

                            // 获取远程代码
                            if let Err(e) = fetch_code(&p, lower.clone()).await {
                                eprintln!("获取代码失败 {}: {}", mono_path, e);
                                continue;
                            }

                            // 更新commit文件中的父提交信息
                            if let Err(e) = set_parent_commit(&work_path).await {
                                eprintln!("设置父提交失败: {}", e);
                            }

                            // 找到并替换旧的工作目录条目
                            for i in 0..ml.works.len() {
                                if ml.works[i].path == mono_path {
                                    ml.works[i] = workdir.clone();
                                    break;
                                }
                            }

                            // 持久化到配置文件
                            let config_file = config::config_file();
                            if let Err(e) = ml.to_toml(config_file) {
                                eprintln!("保存配置失败: {}", e);
                            }

                            // 触发文件系统更新
                            // let fuse = crate::main::get_fuse();
                            // if let Some(fuse) = fuse {
                            //     if let Err(e) = fuse.refresh_overlay(inode, lower).await {
                            //         eprintln!("刷新文件系统失败: {}", e);
                            //     } else {
                            //         println!("工作目录 {} 已更新", mono_path);
                            //     }
                            // }
                        } else {
                            println!("工作目录 {} 无变化", mono_path);
                        }
                    }
                    Err(e) => {
                        eprintln!("获取树结构失败 {}: {}", mono_path, e);
                    }
                }

                // 释放锁
                drop(ml);
            }
        }

        // 休眠一分钟
        println!("完成一轮更新，等待60秒后继续...");
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}

// async fn watch_dir(manager:Arc<Mutex<ScorpioManager>>) {
//     let mut ml = manager.lock().await;
//     for work_dir in &ml.works {
//         let work_dir = fetch(&mut ml, inode, mono_path).await.unwrap();
//         let path = monopath.as_ref().to_str().unwrap().to_string();
//         let p = GPath::from(path);
//         // Get the tree and its hash value, for name dictionary .
//         let tree = fetch_tree(&p).await.unwrap();
//         let workdir = WorkDir {
//             path: p.to_string(),
//             node: inode,
//             hash: tree.id.to_string(),
//         };
//         //work.hash = tree.id.to_string();
//         // the lower path is store file path for remote code version .
//         let store_path = config::store_path();
//         let work_path = PathBuf::from(store_path).join(&workdir.hash);
//         let _lower = work_path.join("lower");
//         fetch_code(&p, _lower).await?;
//         manager.works.push(workdir.clone());
//         let config_file = config::config_file();
//         let _ = manager.to_toml(config_file);

//         // Get the commit information of the previous version and
//         // write it into the commit file.
//         set_parent_commit(&work_path).await?;

//         Ok(workdir)
//     }

// }

/// Mount a dictionary by path , like "/path/to/dic" or "path/to/dic"
async fn mount_handler(
    State(state): State<ScoState>,
    req: axum::Json<MountRequest>,
) -> axum::Json<MountResponse> {
    // transform by GPath , is case of wrong format.
    println!("mount_handler {:?}", req.path);
    let mono_path = GPath::from(req.path.clone()).to_string();
    println!("mount_handler {:?}", mono_path);

    // bool to indicate if it is a temp path for buck2.
    let mut temp_mount = false;
    // get inode by this path .
    let inode = match state.fuse.get_inode(&mono_path).await {
        Ok(a) => a,
        Err(_) => {
            println!("if umount and remount");
            temp_mount = true;
            state
                .fuse
                .dic
                .store
                .add_temp_point(&mono_path)
                .await
                .unwrap()
        }
    };

    // return fail if this inode is mounted.
    if state.fuse.is_mount(inode).await {
        return axum::Json(MountResponse {
            status: FAIL.into(),
            mount: MountInfo::default(),
            message: "The target is mounted.".to_string(),
        });
    }

    let mut ml = state.manager.lock().await;
    if let Err(mounted_path) = ml.check_before_mount(&mono_path) {
        return axum::Json(MountResponse {
            status: FAIL.into(),
            mount: MountInfo::default(),
            message: format!("The {} is already check-out ", mounted_path),
        });
    }
    let store_path = config::store_path();
    // if it is a temp mount , mount it & return the hash and path.
    if temp_mount {
        let temp_hash = {
            let hasher = SHA1::new(mono_path.as_bytes());
            hasher.to_string()
        };

        let store_path = PathBuf::from(store_path).join(&temp_hash);
        println!("temp_mount is true {:?}", store_path);
        let _ = state.fuse.overlay_mount(inode, store_path).await;
        let mount_info = MountInfo {
            hash: temp_hash.clone(),
            path: mono_path.clone(),
            inode,
        };
        ml.works.push(WorkDir {
            path: mono_path,
            node: inode,
            hash: temp_hash,
        });
        let _ = ml.to_toml("config.toml");
        return axum::Json(MountResponse {
            status: SUCCESS.into(),
            mount: mount_info,
            message: "Directory mounted successfully".to_string(),
        });
    }
    // fetch the dionary node info from mono.
    let work_dir = fetch(&mut ml, inode, mono_path).await.unwrap();
    let store_path = PathBuf::from(store_path).join(&work_dir.hash);
    // checkout / mount this dictionary.
    println!("last {:?}", store_path);
    let _ = state.fuse.overlay_mount(inode, store_path).await;

    let mount_info = MountInfo {
        hash: work_dir.hash,
        path: work_dir.path,
        inode,
    };
    axum::Json(MountResponse {
        status: SUCCESS.into(),
        mount: mount_info,
        message: "Directory mounted successfully".to_string(),
    })
}
/// Get all mounted dictionary .
async fn mounts_handler(State(state): State<ScoState>) -> axum::Json<MountsResponse> {
    let manager = state.manager.lock().await;
    let re = manager
        .works
        .iter()
        .map(|word_dir| MountInfo {
            hash: word_dir.hash.clone(),
            path: word_dir.path.clone(),
            inode: word_dir.node,
        })
        .collect();

    axum::Json(MountsResponse {
        status: SUCCESS.into(),
        mounts: re,
    })
}

async fn umount_handler(
    State(state): State<ScoState>,
    req: axum::Json<UmountRequest>,
) -> axum::Json<UmountResponse> {
    let handle;
    if let Some(inode) = req.inode {
        handle = state.fuse.overlay_umount_byinode(inode).await;
    } else if let Some(path) = &req.path {
        handle = state.fuse.overlay_umount_bypath(path).await;
    } else {
        return axum::Json(UmountResponse {
            status: FAIL.into(),
            message: "Need a inode or path.".to_string(),
        });
    }
    match handle {
        Ok(_) => {
            if let Some(path) = &req.path {
                let _ = state.manager.lock().await.remove_workspace(path).await;
            } else {
                //todo be path by inode .
                let path = state
                    .fuse
                    .dic
                    .store
                    .find_path(req.inode.unwrap())
                    .await
                    .unwrap();

                let _ = state
                    .manager
                    .lock()
                    .await
                    .remove_workspace(&path.to_string())
                    .await;
            }

            axum::Json(UmountResponse {
                status: SUCCESS.into(),
                message: "Directory unmounted successfully".to_string(),
            })
        }
        Err(err) => axum::Json(UmountResponse {
            status: FAIL.into(),
            message: format!("Umount process error :{}.", err),
        }),
    }
}

async fn config_handler() -> axum::Json<ConfigResponse> {
    let base_url = config::base_url();
    let workspace = config::workspace();
    let store_path = config::store_path();
    let config_info = ConfigInfo {
        mega_url: base_url.to_string(),
        mount_path: workspace.to_string(),
        store_path: store_path.to_string(),
    };

    axum::Json(ConfigResponse {
        status: SUCCESS.into(),
        config: config_info,
    })
}

async fn update_config_handler(
    State(_state): State<ScoState>,
    req: axum::Json<ConfigRequest>,
) -> axum::Json<ConfigResponse> {
    // update the Configration by request.
    let config_info = ConfigInfo {
        mega_url: req.mega_url.clone().unwrap_or_default(),
        mount_path: req.mount_path.clone().unwrap_or_default(),
        store_path: req.store_path.clone().unwrap_or_default(),
    };

    axum::Json(ConfigResponse {
        status: "success".to_string(),
        config: config_info,
    })
}

mod tests {
    use crate::manager::fetch::CheckHash;
    use crate::manager::ScorpioManager;
    use crate::util::config;
    #[tokio::test]
    async fn test_update() {
        if let Err(e) = config::init_config("/root/git/mega/scorpio/scorpio.toml") {
            eprintln!("Failed to load config: {}", e);
            assert!(false);
        }

        let mut manager = ScorpioManager::from_toml(config::config_file()).unwrap();
        manager.check().await;
    }
    #[tokio::test]
    async fn test_tree() {
        
    }
}
