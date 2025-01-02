use reqwest::{Client, Error, Response};

use url::form_urlencoded;

#[tokio::test]
async fn it_adds_two() {
    let client = Client::new();
    let mut params = form_urlencoded::Serializer::new(String::new());

    // 向序列化器中添加键值对
    params.append_pair("action", "search");
    params.append_pair("q", "诛仙");

    // 获取编码后的字符串
    let encoded_params = params.finish();

    // 创建POST请求，并设置请求体和内容类型
    let response = client
        .post("https://www.wcxsw.la/home/search")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(encoded_params)
        .send();

    match response.await {
        Ok(res) => {
            println!("{:#?}", res);
            let string = res.text().await.unwrap();
            println!("response body={}", string);
        }
        Err(e) => {
            println!("{:#?}", e);
        }
    };
}
