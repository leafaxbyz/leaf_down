use leaf_down::loader::book::download;
use leaf_down::loader::log_init;

#[tokio::main]
async fn main() {
    log_init();
    download().await.expect("err");
}
