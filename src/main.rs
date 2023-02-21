mod common;
mod db;
mod http;
mod json_value;
mod models;
mod parse;
mod schema;

#[macro_use]
extern crate diesel;

#[tokio::main]
async fn main() {
    // let mut args = std::env::args();

    // 拍卖ID
    // let paimai_id = args.nth(1).expect("必须传拍卖ID");

    let paimai_id = std::env::args().nth(1).expect("必须传拍卖ID");

    //所属平台（1&.淘宝、2.京东）
    // let belong = args.nth(2).unwrap_or_else(|| "1".to_string());
    let belong = std::env::args().nth(2).unwrap_or_else(|| "1".to_string());

    println!("跟我买车-爬虫! id={}, 平台：{}", paimai_id, belong);

    //https://sf-item.taobao.com/sf_item/675747836997.htm
    //https://sf-item.taobao.com/auction.htm?spm=a2129.7388457.p1000.50.2fc47e367xEfWY&id=675425091824
    // let url = "https://sf-item.taobao.com/sf_item/678431542538.htm";

    // 京东   https://paimai.jd.com/289011596
    // let url = "https://paimai.jd.com/289011596";
    // let belong = 1; //所属平台（1.淘宝、2.京东）

    if belong.eq("1") {
        let url = "https://sf-item.taobao.com/sf_item/{}.htm".replace("{}", &paimai_id);
        let url = url.as_str();

        // 抓取
        // let url = "https://sf-item.taobao.com/sf_item/679604587720.htm";
        let result = crate::http::http_request(url).await;
        let response = result.unwrap();
        let html = response.html.as_str();
        println!("抓取到的html=========={}", html);
        // let html = include_str!("html/taobao.html");
        let data = crate::parse::taobao_select(html).await;
        if data.is_none() {
            log::error!("解析HTML得不到数据");
            println!("解析HTML得不到数据");
        }
        println!("抓到的数据：{:#?}", data);
        let data = data.unwrap();
        insert_table(data, url); //插入到表
    } else {
        let url = format!("https://paimai.jd.com/{}", paimai_id);
        let url = url.to_owned();

        let data = crate::parse::jd_select(&paimai_id).await;
        if data.is_none() {
            log::error!("京东法拍接口取数据出错");
        }
        println!("京东接口返回的数据：{:#?}", data);
        let data = data.unwrap();
        insert_table(data, &url); //插入到表
    }
}

pub fn insert_table(data: crate::parse::Reptile, url: &str) {
    // 开始插入到表
    use crate::models::lawsuit_reptile_model::NewLawsuitReptile;
    use crate::models::lawsuit_reptile_photo_model::NewLawsuitReptilePhoto;
    use diesel::data_types::Cents; //i64

    // 两种字符转数字的方法 ￥241.00  这里单位为分，要×100;
    // let current_price = data.current_price.parse::<i64>().expect("字符串转i64出错");
    use std::str::FromStr;
    let current_price = f64::from_str(data.current_price.as_str())
        .map_err(|_| "i64转换失败？？")
        .unwrap();
    let current_price = (current_price * 100.) as i64;
    let price_base = (data.price_base * 100.) as i64;
    let assess_price = (data.assess_price * 100.) as i64;
    let margin = (data.margin * 100.) as i64;

    //毫秒和秒相差1000,但这样转换,不知道为何少了8个小时,所以主动加上去 8*3600
    let start_time =
        chrono::prelude::NaiveDateTime::from_timestamp(data.start_time / 1000 + 8 * 3600, 0);
    let end_time =
        chrono::prelude::NaiveDateTime::from_timestamp(data.end_time / 1000 + 8 * 3600, 0);
    let now_date_time = crate::common::now_naive_date_time();

    let new_data = NewLawsuitReptile {
        title: data.title,
        price_base: Cents(price_base),       //起拍价
        current_price: Cents(current_price), //当前价
        assess_price: Cents(assess_price),   //评估价
        margin: Cents(margin),               //保证金
        start_time: Some(start_time),
        end_time: Some(end_time),
        address: Some(data.address),             //标地物详细地址
        disposal_unit: Some(data.disposal_unit), //处置单位:所属法院
        external_url: Some(url.to_string()),
        belong: Some(data.belong), //所属平台（1.淘宝、2.京东）
        stage: Some(data.stage),   //拍卖阶段（一拍、二拍、变卖、撤回）
        status: 1,                 //状态（1待开拍、2竞拍中、已结束:3成交，4流拍、0无效或撤回）
        create_time: Some(now_date_time),
    };

    let insert_id = new_data.insert();
    println!("插入ID:{}", insert_id);

    let photos = data.photos;
    if !photos.is_empty() {
        let mut first = 1;
        let mut front_cover = true;
        for photo in photos {
            if first != 1 {
                front_cover = false;
            }
            let insert_photo = NewLawsuitReptilePhoto {
                lrid: insert_id,
                external_small: Some(photo.external_small),
                external_middle: Some(photo.external_middle),
                external_original: Some(photo.external_original),
                front_cover: Some(front_cover),
            };
            insert_photo.insert();
            first += 1;
        }
    }
}
