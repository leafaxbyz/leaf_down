use crate::searcher::res_config::{read_res, ResConfig};
use log::{error, info};
use scraper::{Html, Selector};
use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display};
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};

pub async fn parse() -> Result<(), Box<dyn Error>> {
    let res_config = read_res()?;
    info!("读取配置文件成功  {:?}", res_config);

    match parse_book(res_config).await {
        Ok(_) => Ok(()),
        Err(err) => {
            error!("parse book err{:?}", err);
            Err(err)
        },
    }
}

pub async fn parse_book(parse_rule: ResConfig) -> Result<(), Box<dyn Error>> {
    let url = &parse_rule.book_url;
    let res_body = reqwest::get(url).await?.text().await?;
    let catalogs = parse_catalog(&res_body, &parse_rule)?;
    let path = "book.txt";
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;

    let mut writer = BufWriter::new(file);
    // 保存每一个章节
    for catalog in catalogs {
        let chapter = parse_character(catalog, &parse_rule).await?;
        save_data(&chapter, &mut writer)?;
        info!("章节名=[{}] 已完成下载", chapter.title);
    }
    Ok(())
}

// 解析目录
fn parse_catalog(html: &str, res_config: &ResConfig) -> Result<Vec<Catalog>, Box<dyn Error>> {
    let document = Html::parse_document(html);
    let catalog_selector = Selector::parse(res_config.catalog_selector.as_str()).unwrap();

    let mut catalogs: Vec<Catalog> = Vec::new();

    for element in document.select(&catalog_selector) {
        let title = element.text().collect::<Vec<_>>().join("");
        let path = element.attr("href").unwrap_or_else(|| "");

        let catalog = Catalog {
            title,
            url: res_config.host.to_string() + path,
        };
        catalogs.push(catalog)
    }
    info!("章节数量={}", catalogs.len());
    Ok(catalogs)
}

// 解析章节
async fn parse_character(
    catalog: Catalog,
    res_config: &ResConfig,
) -> Result<Chapter, Box<dyn Error>> {
    info!("章节url={}", catalog.url);
    let resp = reqwest::get(catalog.url).await?;
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
        Err(err) => Err(Box::new(MyError {
            description: err.to_string(),
        })),
    }
}

// 保持数据到文件
fn save_data(chapter: &Chapter, writer: &mut BufWriter<File>) -> Result<(), Box<dyn Error>> {
    let write_content = format!("\n {} \n {}", chapter.title, chapter.content);
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

#[derive(Debug)]
pub struct MyError {
    description: String,
}

// 实现 std::error::Error 和 std::fmt::Display trait
impl Error for MyError {
    fn description(&self) -> &str {
        &self.description
    }
}

impl Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}
