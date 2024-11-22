use booksearch::searcher::book::parse;
use booksearch::searcher::log_init;
use log::info;

#[tokio::main]
async fn main() {
    log_init();
    info!("This is an info message.");

    parse().await.expect("err");
}
