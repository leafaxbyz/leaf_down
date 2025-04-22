use leaf_down_lib::loader::book::download;
use leaf_down_lib::loader::init::log_init;

#[tokio::test]
async fn test_down() {
    log_init();
    download().await.expect("")
}
