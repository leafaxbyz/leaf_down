use booksearch::searcher::book::download;
use booksearch::searcher::log_init;

#[tokio::main]
async fn main() {
    log_init();
    download().await.expect("err");
}
