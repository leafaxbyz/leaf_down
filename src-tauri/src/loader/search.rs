use crate::loader::err::CustomError;
use crate::loader::res_config::{BookLink, ResConfig};
use log::{error, info};
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
        parse_link(config, res.as_str())?;
        info!("res={:?}", res);
    }

    Ok(Vec::new())
}

pub fn parse_link(config: ResConfig, content: &str) -> Result<Vec<BookLink>, CustomError> {
    info!("Parsing link: {}", content);
    match Selector::parse(config.link_selector.as_str()) {
        Ok(selector) => {
            let document = Html::parse_document(content);
            let  elements = document.select(&selector);
            // let mut content = String::from("");
            let mut links: Vec<BookLink> = Vec::new();
            for element in elements {
                let name = element.text().collect::<Vec<_>>().join("");
                let url = element.attr("href").unwrap();
                let book_link = BookLink {
                    url: url.to_string(),
                    name,
                };
                links.push(book_link);
            }
            info!("book_link: {:?}", links);
            Ok(links)
        }
        Err(err) => {
            error!("select error {:?}", err);
            Err(CustomError::Err("没有解析到书籍".to_string()))
        }
    }
}
