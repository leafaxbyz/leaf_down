use log::{error, info};
use crate::loader::err::CustomError;
use crate::loader::res_config::{BookLink, ResConfig};
use reqwest::Client;
use scraper::{Html, Selector};

// 搜索数据
pub async fn search_books(
    url_list: Vec<ResConfig>,
    book_name: String,
) -> Result<Vec<BookLink>, CustomError> {
    let client = Client::new();

    for config in url_list {
        let url = config.search_url.replace("{book_name}", book_name.as_str());
        let res = client.get(&url).send().await?.text().await?;
        parse_link(config,res.as_str())?;
        info!("res={:?}", res);
    }

    Ok(Vec::new())
}

pub fn parse_link(config: ResConfig, content: &str) -> Result<BookLink, CustomError> {
    info!("Parsing link: {}", content);
    match Selector::parse(config.link_selector.as_str()) {
        Ok(selector) => {
            let document = Html::parse_document(content);
            let mut elements = document.select(&selector);
            // let mut content = String::from("");
            let a = elements.next().unwrap();
            let name = a.text().collect::<Vec<_>>().join("");
            let url = a.attr("href").unwrap();
            let book_link = BookLink { url: url.to_string(), name };
            info!("book_link: {:?}", book_link);
            Ok(book_link)
        }
        Err(err) => {
            error!("select error {:?}", err);
            Err(CustomError::Err("没有解析到书籍".to_string()))}
    }
}
