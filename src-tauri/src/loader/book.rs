use crate::loader::err::CustomError;
use crate::loader::res_config::{read_res, ResConfig};
use log::{error, info};
use reqwest::Client;
use scraper::{Html, Selector};
use std::error::Error;
use std::fmt::Debug;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::time::{Duration, Instant};

// 下载书籍
pub async fn download() -> Result<(), CustomError> {
    let res_config = read_res()?;
    info!("读取配置文件成功  {:?}", res_config);

    let start = Instant::now();
    match parse_book(res_config).await {
        Ok(_) => {
            let elapsed = Instant::now().duration_since(start);
            info!("下载完成, 共耗时{}秒", elapsed.as_secs());
            Ok(())
        }
        Err(err) => {
            error!("parse book err{:?}", err);
            Err(err)
        }
    }
}

// 解析并下载书籍
pub async fn parse_book(res_config: ResConfig) -> Result<(), CustomError> {
    let url = &res_config.book_url;
    let client = Client::builder()
        .timeout(Duration::from_secs(10)) // 设置超时时间为5秒
        .build()
        .map_err(|err| {
            error!("构建客户端错误{:?}", err);
            CustomError::RequestError(err)
        })?;
    let res = client.get(url).send().await.map_err(|err| {
        error!("request err {:?}", err);
        CustomError::RequestError(err)
    })?;
    let res_body = res.text().await.map_err(|err| {
        error!("response to text err {:?}", err);
        CustomError::RequestError(err)
    })?;

    let book_name = parse_name(&res_body, &res_config).map_err(|err| {
        error!("parse name err {:?}", err);
        CustomError::Err(err.to_string())
    })?;
    info!("已获取到书籍名称={}", book_name);
    let full_path = format!("{}/{}", &res_config.save_dir, book_name);

    // 解析所有章节
    let catalogs = parse_catalog(&res_body, &res_config).map_err(|err| {
        error!("parse catalog err {:?}", err);
        CustomError::Err(err.to_string())
    })?;

    // 下载所有章节
    let result = down_catalogs(&res_config, &full_path, catalogs, &client)
        .await
        .map_err(|err| {
            error!("down_catalogs err {:?}", err);
            CustomError::Err(err.to_string())
        })?;
    Ok(result)
}

// 获取书籍名称
fn parse_name(html: &str, res_config: &ResConfig) -> Result<String, Box<dyn Error>> {
    let name_selector = Selector::parse(res_config.name_selector.as_str()).unwrap();
    let book_name = match Html::parse_document(&html).select(&name_selector).next() {
        Some(e) => (e.text().collect::<Vec<_>>().join("") + ".txt").to_string(),
        None => "book.txt".to_string(),
    };
    Ok(book_name)
}

// 解析目录
fn parse_catalog(html: &str, res_config: &ResConfig) -> Result<Vec<Catalog>, Box<dyn Error>> {
    let document = Html::parse_document(html);
    let catalog_selector = Selector::parse(res_config.catalog_selector.as_str()).unwrap();

    let mut catalogs: Vec<Catalog> = Vec::new();

    for element in document.select(&catalog_selector) {
        let title = element.text().collect::<Vec<_>>().join("");
        let path = element.attr("href").unwrap();

        let catalog = Catalog {
            title,
            url: res_config.host.to_string() + path,
        };
        catalogs.push(catalog)
    }
    info!("章节数量={}", catalogs.len());
    Ok(catalogs)
}

// 下载左右章节
async fn down_catalogs(
    res_config: &ResConfig,
    full_path: &str,
    catalogs: Vec<Catalog>,
    client: &Client,
) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(&res_config.save_dir)?;
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(full_path)?;

    let mut writer = BufWriter::new(file);
    // 保存每一个章节
    for catalog in catalogs {
        let chapter = parse_character(catalog, &res_config, &client).await?;
        save_data(&chapter, &mut writer)?;
        info!("章节名=[{}] 已完成下载", chapter.title);
    }
    Ok(())
}

// 解析每个章节
async fn parse_character(
    catalog: Catalog,
    res_config: &ResConfig,
    client: &Client,
) -> Result<Chapter, Box<dyn Error>> {
    info!("章节url={}", catalog.url);
    let resp = client.get(catalog.url).send().await?;
    let resp_body = resp.text().await?;
    let document = Html::parse_document(&resp_body);
    match Selector::parse(res_config.chapter_selector.as_str()) {
        Ok(value) => {
            let elements = document.select(&value);
            let mut content = String::from("");
            for element in elements {
                let p_content = element.text().collect::<Vec<_>>().join("");
                content = format!("{} \n {}", content, p_content);
            }
            let chapter = Chapter {
                title: catalog.title,
                content,
            };
            Ok(chapter)
        }
        Err(err) => Err(Box::new(CustomError::Err(err.to_string()))),
    }
}

// 保持数据到文件
fn save_data(chapter: &Chapter, writer: &mut BufWriter<File>) -> Result<(), Box<dyn Error>> {
    let write_content = format!("\n \n{} \n {}", chapter.title, chapter.content);
    writer.write_all(write_content.as_bytes())?;
    writer.flush()?;
    Ok(())
}

// 目录
#[derive(Debug)]
struct Catalog {
    url: String,
    title: String,
    // selector: String,
}

// 章节
#[derive(Debug)]
struct Chapter {
    title: String,   // 标题
    content: String, // 内容
}
