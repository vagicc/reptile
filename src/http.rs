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
    headers.insert("user-agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/102.0.5005.167 YaBrowser/22.7.5.1036 (beta) Yowser/2.5 Safari/537.36".parse().unwrap());
    headers.insert("sec-ch-ua-platform", "Linux".parse().unwrap());
    headers.insert("cookie", "thw=cn; enc=wbRsEJgvhPHBD/N0N42Jlhh0KXd5aV/xsplYBjWdm25pmESD2YZ23D63x9fE6Mp8G+qYpFQihRL26CFssYUDNg==; t=136828193e6b78f438f8a8ef53c581dc; useNativeIM=false; wwUserTip=false; xlly_s=1; cookie2=29e21b311094011a6981d202ce25088d; _tb_token_=3333eeb547e7b; _samesite_flag_=true; _m_h5_tk=0965011027c42a8e88e80017217b1807_1674185537124; _m_h5_tk_enc=067c38faf1cce8b2e45aa57d97d44d67; mt=ci=0_0; cna=e9dQHKn0KjQCAXjnP1KaPg6c; sgcookie=E100h16TsxVpUEKi74rVDZez3WGHA4SJ+7FWLRUK0Z7cgEmxoVLVtkPdtIbmXAy2JTUYW2EeaZz1YGVHmhHkOjL9gt8cZRkcpBiuW+SxNSPHSPfV4sfpKpH4Ikz/JrZGMbVI; unb=56757547; uc3=nk2=AHdLbOA=&id2=Vvj5o6K0YrY=&lg2=URm48syIIVrSKA==&vt3=F8dCvjMoSqiKL8xINAA=; csg=69297fdc; lgc=cmb7d; cancelledSubSites=empty; cookie17=Vvj5o6K0YrY=; dnk=cmb7d; skt=159d9615375f9231; existShop=MTY3NDE3NzE4OA==; uc4=id4=0@VH787f9j2cxUrCMXV8EbUeuEeg==&nk4=0@Ahn7yQiHu3/U49lln7Qg3Q==; publishItemObj=Ng==; tracknick=cmb7d; _cc_=UIHiLt3xSw==; _l_g_=Ug==; sg=d74; _nk_=cmb7d; cookie1=Vvit2OMSD7FWc9oe/ldU+hycVtQPcH/kh5K7sGm5bV8=; uc1=cookie14=UoezS6pj2rytBQ==&existShop=false&pas=0&cookie21=W5iHLLyFfX5YzvvQHefkmw==&cookie15=WqG3DMC9VAQiUQ==&cookie16=W5iHLLyFPlMGbLDwA+dvAGZqLg==; tfstk=cSvlBPiZp5qQNdDplU6SB-oVaKRhaUc1zhtD3mPgSCDJfoJG4sXbg5kqAYQq54PC.; l=fB_O9jWmLQ-LyVgEBOfCFurza7yepIR4DuPzaNbMi9X197futdQU8HZcbosHn3QKKTfVfetrVcvmJdUvWczUzxGAHeLNJmVcYxv9-bpU-L5..; isg=BJ-fqbzLAJ8R3gW0NqnzfxaQLvopBPOmVVY3gTHsCc4LwL1COdYM99HSh1i-g8se".parse().unwrap());

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
