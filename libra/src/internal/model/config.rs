use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "config")]
///libra.db 里面，可以看见config表
/// 对应libra config -l 命令看见的所有配置项
/// libra config --add remote.origin.url http://localhost:58001/third-part/lfs.git
/// 类似于这样来添加配置
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    // [configuration "name"]=>[remote "origin"]
    pub configuration: String, // configuration option
    pub name: Option<String>,  // name of the configuration (optionally)
    pub key: String,
    pub value: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
