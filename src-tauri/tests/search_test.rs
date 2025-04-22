use leaf_down_lib::loader::init::log_init;
use leaf_down_lib::loader::res_config::ResConfig;
use leaf_down_lib::loader::search::search_books;

#[tokio::test]
async fn test_search() {
    log_init();

    let url1 = String::from("https://www.biqu70.cc/s?q={book_name}");
    let config = ResConfig {
        host: "".to_string(),
        search_url: url1,
        link_selector: ".bookname a".to_string(),
        book_url: "".to_string(),
        catalog_selector: "".to_string(),
        chapter_selector: "".to_string(),
        name_selector: "".to_string(),
        save_dir: "".to_string(),
    };
    let links = search_books(vec![config], "诛仙".to_string())
        .await
        .unwrap();
    println!("{:?}", links);
}
