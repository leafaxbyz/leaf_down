use crate::loader::err::CustomError;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::{env, fs};

#[derive(Debug, Deserialize, Serialize)]
pub struct ResConfig {
    pub host: String,
    pub search_url: String,
    pub link_selector: String,
    pub book_url: String,
    pub catalog_selector: String,
    pub chapter_selector: String,
    pub name_selector: String,
    pub save_dir: String,
}

// 书籍
#[derive(Debug, Deserialize, Serialize)]
pub struct BookLink {
    pub url: String,  // 路径
    pub name: String, // 书名
}

pub fn read_res() -> Result<ResConfig, CustomError> {
    // 配置文件目录
    let file_path = "res_2.json";

    let cwd = env::current_dir().map_err(|e| {
        error!("Can not access current working directory: {}", e);
        CustomError::ConfigReadErr(e)
    })?;
    info!("当前工作路径: {}", cwd.display());

    // 从文件中读取数据
    let mut file = fs::File::open(file_path).map_err(|e| {
        error!("open file err: {}, file_path= {}", e, file_path);
        CustomError::ConfigReadErr(e)
    })?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| {
        error!("read file err: {}, file_path = {}", e, file_path);
        CustomError::ConfigReadErr(e)
    })?;

    // 将字符串解析为Person结构体
    let config = serde_json::from_str(&contents).map_err(|e| {
        error!("parse config err: {}, file_path = {}", e, file_path);
        CustomError::ConfigParseErr(e)
    })?;
    Ok(config)
}
