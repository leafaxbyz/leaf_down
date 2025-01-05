use leaf_down_lib::loader::init::log_init;
use leaf_down_lib::loader::search::search_books;
use leaf_down_lib::loader::res_config::ResConfig;

#[tokio::test]
async fn test_parse() {
    log_init();

    let url1 = String::from("https://www.wcxsw.la/home/search?action=search&q={book_name}");
    let config = ResConfig{
        host: "".to_string(),
        search_url: url1,
        link_selector: "#hotcontent .lll .item-cover a".to_string(),
        book_url: "".to_string(),
        catalog_selector: "".to_string(),
        chapter_selector: "".to_string(),
        name_selector: "".to_string(),
        save_dir: "".to_string(),
    };
    let _ = search_books(vec![config], "诛仙".to_string()).await;
}
