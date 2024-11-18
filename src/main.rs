use booksearch::parser::book::parse;

#[tokio::main]
async fn main() {
    parse().await
}
