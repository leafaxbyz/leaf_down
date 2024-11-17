mod core;
use crate::core::parse;

#[tokio::main]
async fn main() {
    parse().await;
}

