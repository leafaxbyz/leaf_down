use serde::{Deserialize, Serialize};
use std::io::Read;
use std::{env, fs};

#[derive(Debug, Deserialize, Serialize)]
pub struct ResConfig {
    pub host: String,
    pub book_url: String,
    pub catalog_selector: String,
    pub chapter_selector: String,
}

pub fn read_res() -> Result<ResConfig, Box<dyn std::error::Error>> {
    // 假设你有一个包含JSON数据的文件
    let file_path = "res_config.json";
    let cwd = env::current_dir()?;
    println!("Current working directory: {:?}", cwd);

    // 从文件中读取数据
    let mut file = fs::File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // 将字符串解析为Person结构体
    let config: ResConfig = serde_json::from_str(&contents)?;
    Ok(config)
}
