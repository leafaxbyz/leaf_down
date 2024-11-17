use crate::core::parser::ParseRule;

mod parser;

pub async fn parse(){
    let parse_rule = ParseRule {
        host: "https://www.xzmncy.com".to_string(),
        book_url: "https://www.xzmncy.com/list/35830/".to_string(),
        catalog_selector: "#list a".to_string(),
        chapter_selector: "#htmlContent p".to_string(),
    };

    match parser::parse_book(parse_rule).await {
        Ok(_) => {}
        Err(err) => {
            println!("解析书籍错误,{}", err)
        }
    }
}