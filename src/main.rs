use booksearch::searcher::book::parse;

#[tokio::main]
async fn main() {
    parse().await.expect("err");
}
