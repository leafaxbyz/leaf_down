use booksearch::searcher::book::download;
use booksearch::searcher::log_init;

#[tokio::test]
async fn test() {
    log_init();
    let _ = download().await.expect("failed to download");
}
