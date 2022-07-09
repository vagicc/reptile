use serde::Deserialize;
use serde_json::{Map, Value};

#[derive(Debug, Deserialize, Clone)]
pub struct DisposalUnit {
    pub code: i8,
    pub data: Option<Unit>,
    pub message: String,
    pub status: i8,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Unit {
    pub shopName: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JDDetail {
    pub code: i8,
    pub data: Option<JDDetailData>,
    pub message: String,
    pub status: i8,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JDDetailData {
    pub endTime: i64,                                  //结束时间戳 ,毫秒
    pub ensurePrice: f64,                              //保证金
    pub judicatureBasicInfoResult: Map<String, Value>, //变卖时=里面有，市场价  也许是  “评估价”
    pub paimaiImageResultList: Vec<Image>,             //相册
    pub paimaiTimes: i8,                               //拍卖阶段   1一拍  2.二拍    4.变卖
    pub priceLowerOffset: f32,                         //加价幅度
    pub productAddressResult: Address,                 //标的物所在地
    pub startPrice: f64,                               //起拍价
    pub assessmentPrice: f64,                          //评估价
    pub startTime: i64,                                //开拍时间戳 ,毫秒
    pub title: String,                                 //标题
    pub courtVendorId: i64,                            //处置单位:所属法院 的 shopId
}

#[derive(Debug, Deserialize, Clone)]
pub struct Image {
    /* 
    imagePath：jfs/t1/197788/4/24657/190942/62ba6399Ef8021fb5/f83aa4bb4dd19449.jpg
    相册小图：https://img10.360buyimg.com/n5/s50x50_jfs/t1/197788/4/24657/190942/62ba6399Ef8021fb5/f83aa4bb4dd19449.jpg
    相册中图：https://img12.360buyimg.com/n0/s350x350_jfs/t1/197788/4/24657/190942/62ba6399Ef8021fb5/f83aa4bb4dd19449.jpg
    详情原图：https://img30.360buyimg.com/imgw/s1000x750_jfs/t1/16936/14/17249/3950592/62ba63a0Ec32a355c/25a3a813085364d9.jpg  
    */
    pub imagePath: String,
    pub skuId: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Address {
    pub address: String,
    pub city: String,
    pub county: String,
    pub province: String,
}
