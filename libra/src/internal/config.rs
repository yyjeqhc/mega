use std::collections::HashSet;
use std::mem::swap;

use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, QueryFilter};

use crate::internal::db::get_db_conn_instance;
use crate::internal::head::Head;
use crate::internal::model::config;
use crate::internal::model::config::Model;

use super::model::config::ActiveModel;

pub struct Config;
///origin https://...
pub struct RemoteConfig {
    pub name: String,
    pub url: String,
}
#[allow(dead_code)]
pub struct BranchConfig {
    pub name: String,
    pub merge: String,
    pub remote: String,
}

impl Config {
    // todo accept a db connect or a transaction from outside
    ///数据库简单的插入一行
    pub async fn insert(configuration: &str, name: Option<&str>, key: &str, value: &str) {
        let db = get_db_conn_instance().await;
        let config = ActiveModel {
            configuration: Set(configuration.to_owned()),
            name: Set(name.map(|s| s.to_owned())),
            key: Set(key.to_owned()),
            value: Set(value.to_owned()),
            ..Default::default()
        };
        config.save(db).await.unwrap();
    }

    // Update one configuration entry in database using given configuration, name, key and value
    ///数据库操作，更新一行
    pub async fn update(configuration: &str, name: Option<&str>, key: &str, value: &str) -> Model {
        let db = get_db_conn_instance().await;
        let mut config: ActiveModel = config::Entity::find()
            .filter(config::Column::Configuration.eq(configuration))
            .filter(match name {
                Some(str) => config::Column::Name.eq(str),
                None => config::Column::Name.is_null(),
            })
            .filter(config::Column::Key.eq(key))
            .one(db)
            .await
            .unwrap()
            .unwrap()
            .into();
        config.value = Set(value.to_owned());
        config.update(db).await.unwrap()
    }

    ///数据库配置竟然可以重复，除了value不一样，根据这几个属性查询所有行
    async fn query(configuration: &str, name: Option<&str>, key: &str) -> Vec<Model> {
        let db = get_db_conn_instance().await;
        config::Entity::find()
            .filter(config::Column::Configuration.eq(configuration))
            .filter(match name {
                Some(str) => config::Column::Name.eq(str),
                None => config::Column::Name.is_null(),
            })
            .filter(config::Column::Key.eq(key))
            .all(db)
            .await
            .unwrap()
    }

    /// Get one configuration value
    /// 可能会有重复，但是只需要第一个
    pub async fn get(configuration: &str, name: Option<&str>, key: &str) -> Option<String> {
        let values = Self::query(configuration, name, key).await;
        values.first().map(|c| c.value.to_owned())
    }

    /// Get remote repo name by branch name
    /// - You may need to `[branch::set-upstream]` if return `None`
    /// libra branch --set-upstream-to=origin/master
    /// 只是为了查看当前分支关联的远程value，也就是origin或者其他的东西
    pub async fn get_remote(branch: &str) -> Option<String> {
        // e.g. [branch "master"].remote = origin
        Config::get("branch", Some(branch), "remote").await
    }

    /// Get remote repo name of current branch
    /// - `Error` if `HEAD` is detached
    /// 查看当前分支的远程关联名称，返回remote_name origin，以便后面查询url
    pub async fn get_current_remote() -> Result<Option<String>, ()> {
        match Head::current().await {
            Head::Branch(name) => Ok(Config::get_remote(&name).await),
            Head::Detached(_) => {
                eprintln!("fatal: HEAD is detached, cannot get remote");
                Err(())
            }
        }
    }

    ///给定远程名称，查询实际的url。
    /// 比如 origin ,返回https://...
    pub async fn get_remote_url(remote: &str) -> String {
        println!("remote is {:?}", remote);
        let a = Config::list_all().await;
        for (k, v) in a {
            println!("list {}: {}", k, v);
        }
        // let b = Config::branch_config(remote).await.unwrap();
        // println!("{:?} {:?}",b.remote,b.name);
        // let c = Config::all_remote_configs().await;
        // for r in c {
        //     println!("remote config: {} {}", r.name, r.url);
        // }
        // return "http://localhost:58001/".to_string();
        match Config::get("remote", Some(remote), "url").await {
            Some(url) => url,
            None => panic!("fatal: No URL configured for remote '{}'.", remote),
        }
    }

    /// return `None` if no remote is set
    /// 和get_current_remote配套使用，查看当前分支的远程，再查询远程实际的url
    pub async fn get_current_remote_url() -> Option<String> {
        match Config::get_current_remote().await.unwrap() {
            Some(remote) => Some(Config::get_remote_url(&remote).await),
            None => None,
        }
    }

    /// Get all configuration values
    /// - e.g. remote.origin.url can be multiple
    /// 只是为了获取某个配置的所有值，如 remote.origin.url
    pub async fn get_all(configuration: &str, name: Option<&str>, key: &str) -> Vec<String> {
        Self::query(configuration, name, key)
            .await
            .iter()
            .map(|c| c.value.to_owned())
            .collect()
    }

    /// Get literally all the entries in database without any filtering
    /// 不做筛选，返回所有配置
    pub async fn list_all() -> Vec<(String, String)> {
        let db = get_db_conn_instance().await;
        config::Entity::find()
            .all(db)
            .await
            .unwrap()
            .iter()
            .map(|m| {
                (
                    match &m.name {
                        Some(n) => m.configuration.to_owned() + "." + n + "." + &m.key,
                        None => m.configuration.to_owned() + "." + &m.key,
                    },
                    m.value.to_owned(),
                )
            })
            .collect()
    }

    /// Delete one or all configuration using given key and value pattern
    /// 删除某个配置项，必须是完全配置，比如remote.origin.url，参数是remote.url，就找不到
    pub async fn remove_config(
        configuration: &str,
        name: Option<&str>,
        key: &str,
        valuepattern: Option<&str>,
        delete_all: bool,
    ) {
        let db = get_db_conn_instance().await;
        let entries: Vec<Model> = Self::query(configuration, name, key).await;
        for e in entries {
            let _res = match valuepattern {
                Some(vp) => {
                    if e.value.contains(vp) {
                        e.delete(db).await
                    } else {
                        continue;
                    }
                }
                None => e.delete(db).await,
            };
            if !delete_all {
                break;
            }
        }
    }

    /// Delete all the configuration entries using given configuration field (--remove-section)
    // pub async fn remove_by_section(configuration: &str) {
    //     unimplemented!();
    // }
    ///输入是origin这样的名称
    pub async fn remove_remote(name: &str) -> Result<(), String> {
        let db = get_db_conn_instance().await;
        let remote = config::Entity::find()
            .filter(config::Column::Configuration.eq("remote"))
            .filter(config::Column::Name.eq(name))
            .all(db)
            .await
            .unwrap();
        if remote.is_empty() {
            return Err(format!("fatal: No such remote: {}", name));
        }
        for r in remote {
            let r: ActiveModel = r.into();
            r.delete(db).await.unwrap();
        }
        Ok(())
    }

    ///libra remote show 或者libra remote -v的时候，进行展示，
    /// 但是，如果name为空，直接崩溃
    pub async fn all_remote_configs() -> Vec<RemoteConfig> {
        let db = get_db_conn_instance().await;
        let remotes = config::Entity::find()
            .filter(config::Column::Configuration.eq("remote"))
            .all(db)
            .await
            .unwrap();
        //有必要进行去重再遍历嘛，直接变量Vec就行了
        //
        remotes
            .into_iter()
            .map(|remote| {
                RemoteConfig {
                    //要考虑name为空，会直接崩溃
                    name: remote.name.as_ref().unwrap().clone(),
                    url: remote.value,
                }
            })
            .collect()

        // let remote_names = remotes
        //     .iter()
        //     .map(|remote| remote.name.as_ref().unwrap().clone())
        //     .collect::<HashSet<String>>();

        // remote_names
        //     .iter()
        //     .map(|name| {
        //         let url = remotes
        //             .iter()
        //             .find(|remote| remote.name.as_ref().unwrap() == name)
        //             .unwrap()
        //             .value
        //             .to_owned();
        //         RemoteConfig {
        //             name: name.to_owned(),
        //             url,
        //         }
        //     })
        //     .collect()
    }

    ///fetch的时候使用，传入那么，比如origin
    /// 是够应该更精确一点。如果不是remote.origin.url呢
    /// 而是remote.origin.hello呢，因为不是使用remote add添加
    /// 而是直接config添加，不过不会崩溃，也不影响
    pub async fn remote_config(name: &str) -> Option<RemoteConfig> {
        let db = get_db_conn_instance().await;
        let remote = config::Entity::find()
            .filter(config::Column::Configuration.eq("remote"))
            .filter(config::Column::Name.eq(name))
            .one(db)
            .await
            .unwrap();
        remote.map(|r| RemoteConfig {
            name: r.name.unwrap(),
            url: r.value,
        })
    }

    ///libra branch --set-upstream-to=origin/master
    ///这样设置的话，确实只有两项
    /// 输入参数是分支名称，返回查询到的branch配置
    pub async fn branch_config(name: &str) -> Option<BranchConfig> {
        let db = get_db_conn_instance().await;
        let config_entries = config::Entity::find()
            .filter(config::Column::Configuration.eq("branch"))
            .filter(config::Column::Name.eq(name))
            .all(db)
            .await
            .unwrap();
        if config_entries.is_empty() {
            None
        } else {
            assert_eq!(config_entries.len(), 2);
            // if branch_config[0].key == "merge" {
            //     Some(BranchConfig {
            //         name: name.to_owned(),
            //         merge: branch_config[0].value.clone(),
            //         remote: branch_config[1].value.clone(),
            //     })
            // } else {
            //     Some(BranchConfig {
            //         name: name.to_owned(),
            //         merge: branch_config[1].value.clone(),
            //         remote: branch_config[0].value.clone(),
            //     })
            // }
            let mut branch_config = BranchConfig {
                name: name.to_owned(),
                merge: config_entries[0].value.clone(),
                remote: config_entries[1].value.clone(),
            };
            if config_entries[0].key == "remote" {
                swap(&mut branch_config.merge, &mut branch_config.remote);
            }
            branch_config.merge = branch_config.merge[11..].into(); // cut refs/heads/

            Some(branch_config)
        }
    }
}
