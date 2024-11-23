use booksearch::searcher::book::parse;
use booksearch::searcher::log_init;

#[tokio::main]
async fn main() {
    log_init();
    parse().await.expect("err");
}
