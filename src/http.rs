#[derive(Debug, Clone)]
pub struct Request {
    pub url: String,
    pub base_url: String,
    pub html: String,
}

pub async fn http_request(url: &str) -> Option<Request> {
    let client = reqwest::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();

    // 请求头设置为“火狐浏览器”
    headers.insert("user-agent", "Mozilla/5.0 (X11; Windows x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/98.0.4758.102 Safari/537.36".parse().unwrap());
    headers.insert("sec-ch-ua-platform", "Windows".parse().unwrap());

    let response = client.get(url).headers(headers).send().await;

    if response.is_err() {
        // println!("详情网站访问不了：{}", url);
        log::error!("详情网站访问不了：{}", url);
        return None;
    }

    let response = response.unwrap();

    let base_url = response.url().origin().unicode_serialization();
    let html = response.text().await.expect("转换为文本出错");

    Some(Request {
        url: url.to_string(),
        base_url: base_url,
        html: html,
    })
}

pub async fn http_get(url: &str) -> Option<reqwest::Response> {
    let client = reqwest::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();

    // 请求头设置为“火狐浏览器”
    headers.insert("user-agent", "Mozilla/5.0 (X11; Windows x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/98.0.4758.102 Safari/537.36".parse().unwrap());
    headers.insert("sec-ch-ua-platform", "Windows".parse().unwrap());

    let response = client.get(url).headers(headers).send().await;

    if response.is_err() {
        // println!("详情网站访问不了：{}", url);
        log::error!("网站访问不了：{}", url);
        return None;
    }

    let response = response.unwrap();
    Some(response)
}
