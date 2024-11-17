use scraper::{Html, Selector};
use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display};
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
pub async fn parse_book(parse_rule: ParseRule) -> Result<(), Box<dyn Error>> {
    let url = &parse_rule.book_url;
    // let host = "https://www.xzmncy.com";
    // let index_path = "/list/35830/";
    // let url = host.to_string() + index_path;
    let res_body = reqwest::get(url).await?.text().await?;
    let catalogs = parse_catalog(&res_body, &parse_rule)?;
    let path = "book.txt";
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;

    let mut writer = BufWriter::new(file);
    for catalog in catalogs {
        let chapter = parse_character(catalog, &parse_rule).await?;
        save_data(&chapter, &mut writer)?;
        println!("{} 已完成", chapter.title);
    }
    Ok(())
}

// 解析目录
fn parse_catalog(html: &str, parse_rule: &ParseRule) -> Result<Vec<Catalog>, Box<dyn Error>> {
    let document = Html::parse_document(html);
    let catalog_selector = Selector::parse(parse_rule.catalog_selector.as_str()).unwrap();

    let mut catalogs: Vec<Catalog> = Vec::new();

    for element in document.select(&catalog_selector) {
        let title = element.text().collect::<Vec<_>>().join("");
        let path = element.attr("href").unwrap_or_else(|| "");

        let catalog = Catalog {
            title,
            url: parse_rule.host.to_string() + path,
            // selector: parse_rule.chapter_selector.to_string(),
        };
        catalogs.push(catalog)
    }
    println!("章节数量={}", catalogs.len());
    Ok(catalogs)
}

async fn parse_character(catalog: Catalog, parse_rule: &ParseRule) -> Result<Chapter, Box<dyn Error>> {
    println!("章节url={}", catalog.url);
    let resp = reqwest::get(catalog.url).await?;
    let resp_body = resp.text().await?;
    let document = Html::parse_document(&resp_body);
    match Selector::parse(parse_rule.chapter_selector.as_str()) {
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
        },
        Err(err) => Err(Box::new(MyError{description: err.to_string()}))
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

// 解析规则
#[derive(Debug)]
pub struct ParseRule {
    pub host: String,  // 域名
    pub book_url: String,  // 书籍地址
    pub catalog_selector: String,  // 目录选择器
    pub chapter_selector: String,  // 章节选择器
}

#[derive(Debug)]
pub struct MyError {
    description: String,
}

// 实现 std::error::Error 和 std::fmt::Display trait
impl std::error::Error for MyError {
    fn description(&self) -> &str {
        &self.description
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}



