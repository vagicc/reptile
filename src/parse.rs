#[derive(Debug, Clone)]
pub struct Reptile {
    pub title: String, //标题
    // pub list_img:String,  //封面图-列表图
    pub price_base: f64,       //起拍价
    pub current_price: String, //当前价
    pub assess_price: f64,     //评估价
    pub margin: f64,           //保证金
    pub start_time: i64,       //开拍时间
    pub end_time: i64,         //拍卖结束时间
    pub address: String,       //标的物地址
    pub disposal_unit: String, //处置单位:所属法院
    // pub external_url: String,  //拍卖url
    pub belong: i16,        //所属平台（1.淘宝、2.京东）
    pub stage: String,      //拍卖阶段（一拍、二拍、变卖、撤回）
    pub photos: Vec<Photo>, //相册
}

#[derive(Debug, Clone)]
pub struct Photo {
    pub external_small: String,
    pub external_middle: String,
    pub external_original: String,
}

pub async fn jd_select(paimai_id: &str) -> Option<Reptile> {
    use crate::http::http_get;

    // 京东法拍车详情API接口
    // let url=format!("https://api.m.jd.com/api?appid=paimai&functionId=getProductBasicInfo&body={%22paimaiId%22:{}}",paimai_id);
    let detail_url="https://api.m.jd.com/api?appid=paimai&functionId=getProductBasicInfo&body={%22paimaiId%22:289011596}";
    let url = detail_url.replace("289011596", paimai_id);
    // println!("详情：{} ",url);
    let result = http_get(&url).await;
    if result.is_none() {
        log::error!("京东法拍车接口请求失败,url: {}", url);
        return None;
    }
    let response = result.unwrap();

    use crate::json_value::jd_value::JDDetail;
    let data = response
        .json::<JDDetail>()
        .await
        .expect("接口返回转换数据为json时出错");

    // println!("{:?}", data);
    if data.status != 0 || data.data.is_none() {
        log::error!("接口返回数据无效： {:?}", data);
        return None;
    }

    let detail = data.data.unwrap();
    println!("{:#?}", detail);

    let current_price = detail.startPrice.to_string();
    let address = format!(
        "{} {} {} {}",
        detail.productAddressResult.province,
        detail.productAddressResult.city,
        detail.productAddressResult.county,
        detail.productAddressResult.address
    );

    let mut disposal_unit = String::from(""); //处置单位:所属法院 ========
    let shop_id = format!("{}", detail.courtVendorId);
    let shop_id = shop_id.as_str();
    let unit_url="https://api.m.jd.com/api?appid=paimai&functionId=queryVendorInfo&body={%22vendorId%22:640378,%22publishSource%22:7}";
    let url = unit_url.replace("640378", shop_id);

    let result = http_get(&url).await;
    if result.is_none() {
        log::error!("京东法拍车接口请求失败,url: {}", url);
        return None;
    }
    let response = result.unwrap();

    use crate::json_value::jd_value::DisposalUnit;
    let disposal_data = response
        .json::<DisposalUnit>()
        .await
        .expect("所属法院接口返回转换数据为json时出错");
    if disposal_data.status == 0 && !disposal_data.data.is_none() {
        disposal_unit = disposal_data.data.unwrap().shopName;
    }

    let mut photos_vec: Vec<Photo> = Vec::new();

    //处理相册
    let photos = detail.paimaiImageResultList;
    for img in photos.iter() {
        let small_img = format!("//img10.360buyimg.com/n5/s50x50_{}", img.imagePath);
        let middle_img = format!("//img12.360buyimg.com/n0/s350x350_{}", img.imagePath);
        let original_img = format!("//img30.360buyimg.com/imgw/s1000x750_{}", img.imagePath);
        // println!("相册小图：{}", small_img);
        // println!("相册中图：{}", middle_img);
        // println!("详情原图：{}", original_img);

        photos_vec.push(Photo {
            external_small: small_img,
            external_middle: middle_img,
            external_original: original_img,
        });
    }

    Some(Reptile {
        title: detail.title, //标题
        // pub list_img:String,  //封面图-列表图
        price_base: detail.startPrice,        //起拍价
        current_price: current_price,         //当前价
        assess_price: detail.assessmentPrice, //评估价
        margin: detail.ensurePrice,           //保证金
        start_time: detail.startTime,         //开拍时间
        end_time: detail.endTime,             //拍卖结束时间
        address: address,                     //标的物地址
        disposal_unit: disposal_unit,         //处置单位:所属法院
        //  external_url: String,  //拍卖url
        belong: 2,                               //所属平台（1.淘宝、2.京东）
        stage: get_jd_stage(detail.paimaiTimes), //拍卖阶段（一拍、二拍、变卖、撤回）
        photos: photos_vec,
    })

    // return None;
}

pub fn get_jd_stage(paimai_times: i8) -> String {
    // 京东拍卖阶段   1一拍  2.二拍    4.变卖
    if paimai_times == 1 {
        return "一拍".to_string();
    }
    if paimai_times == 2 {
        return "一拍".to_string();
    }
    if paimai_times == 4 {
        return "一拍".to_string();
    }
    return "重拍".to_string();
}

/// 解析淘宝法拍车详情
/// let html = include_str!("html/taobao.html");
/// taobao_select(html).await;
pub async fn taobao_select(html: &str) -> Option<Reptile> {
    use select::document::Document;
    use select::predicate::{Attr, Class, Name, Predicate};

    // let html = include_str!("html/taobao.html");
    let document = Document::from(html);

    let pm_main = document.find(Class("pm-main")).next();
    if pm_main.is_none() {
        log::error!("查无主体");
        return None;
    }
    let pm_main_node = pm_main.unwrap();
    let status_node = pm_main_node.find(Class("item-status")).next().unwrap();
    let status_string = status_node.text().trim().to_string();
    // println!("拍卖阶段-状态：{}", status_string);

    // let h1_node = pm_main_node.find(Name("h1")).next().unwrap();

    // id="J_ImgBooth" 第一张大图
    let title_node = document.find(Attr("id", "J_ImgBooth")).next().unwrap();
    let title = title_node.attr("alt").unwrap().trim();
    // println!("找到标题：{:?}", title);

    // id="J_UlThumb" 下为所有图片
    let imgs_ul = document.find(Attr("id", "J_UlThumb")).next();
    let imgs_ul_node = imgs_ul.unwrap();

    // 开始时间-结束时间  可以取得开始时间戳data-start与结束时间戳data-end ，但  ---
    /*
    <ul class="pm-bid-eyebrow">
        <li class="J_PItem" id="sf-countdown" data-itemId="675425091824" data-now="1657028404000"
            data-timeToEnd="1685996000" data-timeToStart="-3498004000" data-end="1658714400000"
            data-start="1653530400000">
            <span class="title J_TimeTitle">距结束</span>
            <span
            class="countdown J_TimeLeft"><var>0</var><em>天</em><var>0</var><var>0</var><em>时</em><var>0</var><em>分</em><var>0</var><em>秒</em></span>
            <span id="J_Delay" class="pm-delay"><em class="delayCnt">0</em>次延时</span>
        </li>
    */

    let mut start_time: i64 = 0;
    let mut end_time: i64 = 0;
    //拍卖结束后找不到此ID:sf-countdown
    if let Some(time_node) = document.find(Attr("id", "sf-countdown")).next() {
        let start_time_str = time_node
            .attr("data-start")
            .expect("找不到开始拍卖时间戳")
            .trim();
        start_time = start_time_str.parse::<i64>().unwrap();

        // println!("开始拍卖时间:{:#?}", start_time);
        let end_time_str = time_node
            .attr("data-end")
            .expect("拍卖结束时间戳找不到")
            .trim();
        end_time = end_time_str.parse::<i64>().unwrap();
        // println!("拍卖结束时间戳:{:#?}", end_time);
    }

    // let time_node = document.find(Attr("id", "sf-countdown")).next().unwrap();
    // let start_time = time_node
    //     .attr("data-start")
    //     .expect("找不到开始拍卖时间戳")
    //     .trim();
    // let start_time = start_time.parse::<i64>().unwrap();
    // // println!("开始拍卖时间:{:#?}", start_time);
    // let end_time = time_node
    //     .attr("data-end")
    //     .expect("拍卖结束时间戳找不到")
    //     .trim();
    // let end_time = end_time.parse::<i64>().unwrap();
    // // println!("拍卖结束时间戳:{:#?}", end_time);

    // 价格：id="sf-price"
    let price_node = document
        .find(Attr("id", "sf-price").descendant(Class("pm-current-price")))
        .next()
        .unwrap();
    let price_node = price_node.find(Name("em")).next().unwrap();
    let current_price = price_node.text().trim().replace(",", "");
    // println!("当前价：{}", current_price);

    let margin_node = document
        .find(Attr("id", "submitDeposit").descendant(Name("span")))
        .next();

    let mut margin = String::new();
    if let Some(margin_node) = margin_node {
        let margin_string = margin_node.text();
        let margin_split: Vec<&str> = margin_string.trim().split('¥').collect();
        margin = margin_split.last().expect("切割(保证金)出错").to_string();
    } else {
        println!("已拍卖结束，没未登录交保证HTML");
    }

    // 保证金 margin
    // let margin_node = document
    //     .find(Attr("id", "submitDeposit").descendant(Name("span")))
    //     .next()
    //     .expect("找不到保证金"); //J_HoverShow
    // let margin_string = margin_node.text();
    // let margin_split: Vec<&str> = margin_string.trim().split('¥').collect();

    // let margin = margin_split.last().expect("切割(保证金)出错");
    // println!("保证金:{}", margin);

    // 标的物位置
    let address_node = document.find(Attr("id", "itemAddress")).next().unwrap();
    let address = address_node.text();
    // println!("标的物位置-所在 省 市 区: {}", address);
    let address_node = document
        .find(Attr("id", "itemAddressDetail"))
        .next()
        .unwrap();
    let address_detail = address_node.text();
    // println!("标的物位置-详细地址: {}", address_detail);
    let address = format!("{} {}", address, address_detail);

    // 处置单位disposal_unit
    let unit_node = document
        .find(Class("unit-name"))
        .next()
        .unwrap()
        .find(Name("a"))
        .next()
        .unwrap();
    let disposal_unit = unit_node.text().trim().to_string();
    // println!("处置单位:{}", disposal_unit);

    let mut assess_price = String::new();
    let mut price_base = String::new();

    // 评估价 assess_price
    let price = document.find(Attr("id", "J_HoverShow")).next().unwrap();
    for price_temp in price.find(Name("td")) {
        let price_title = price_temp.find(Class("pay-mark")).next().unwrap();
        let price_title = price_title.text();
        let price_number = price_temp.find(Class("J_Price")).next();
        if price_number.is_none() {
            // println!("路过,{}", price_title);
            continue;
        }
        let price_number = price_number.unwrap();
        let price_number = price_number.text().trim().to_string();
        let price_number = price_number.replace(",", "");
        println!("价格标题：{}，=> {}", price_title, price_number);
        if price_title.eq("评 估 价") {
            assess_price = price_number;
            continue;
        }
        // 起拍价 或 变卖价
        if price_title.eq("起拍价") || price_title.eq("变卖价") {
            price_base = price_number;
            continue;
        }
        // 这里再找“保证金”
        if margin.is_empty() && price_title.eq("保证金") {
            margin = price_number;
            continue;
        }
    }

    let mut photos_vec: Vec<Photo> = Vec::new();

    // 处理相册
    for li_node in imgs_ul_node.find(Name("li")) {
        /*
         <li class="pm-selected J_FristImg">
            <div class="pm-pic pm-s80 ">
                <a href="javascript:void(0);">
                <img src="//img.alicdn.com/bao/uploaded/i4/O1CN01qQAZj729a49qOiAGN_!!0-paimai.jpg_80x80.jpg"
                    alt="琼A7765D金杯牌小型普通客车" />
                </a>
            </div>
        </li>
        */
        let img_node = li_node.find(Name("img")).next().unwrap();
        let img_src = img_node.attr("src").unwrap();
        // println!("相册小图(80x80)：{}", img_src);
        let img_460 = img_src.replace("80x80", "460x460");
        // println!("相册大图(460x460)：{}", img_460);
        let img_960 = img_src.replace("80x80", "960x960");
        // println!("详情原图(960x960)：{}", img_960);
        // 相册小图(80x80)：  //img.alicdn.com/bao/uploaded/i1/O1CN01jKw1eA2CUyc16XOrd_!!0-paimai.jpg_80x80.jpg
        // 相册大图(460x460)：//img.alicdn.com/bao/uploaded/i1/O1CN01jKw1eA2CUyc16XOrd_!!0-paimai.jpg_460x460.jpg
        // 原图，在详情  https://img.alicdn.com/bao/uploaded/i1/O1CN01jKw1eA2CUyc16XOrd_!!0-paimai.jpg_960x960.jpg
        photos_vec.push(Photo {
            external_small: img_src.to_string(),
            external_middle: img_460,
            external_original: img_960,
        });
    }

    let price_base = price_base.parse::<f64>().expect("字符串转f64出错");
    // let assess_price = assess_price.parse::<f64>().expect("字符串转f64出错");
    let assess_price = assess_price.parse::<f64>().unwrap_or_default(); //无评估价时
    let margin = margin.trim().replace(",", "");
    let margin = margin.parse::<f64>().expect("字符串转f64出错");
    Some(Reptile {
        title: title.to_string(), //标题
        // pub list_img:String,  //封面图-列表图
        price_base: price_base,       //起拍价
        current_price: current_price, //当前价
        assess_price: assess_price,   //评估价
        margin: margin,               //保证金
        start_time: start_time,       //开拍时间
        end_time: end_time,           //拍卖结束时间
        address: address,             //标的物地址
        disposal_unit: disposal_unit, //处置单位:所属法院
        //  external_url: String,  //拍卖url
        belong: 1,            //所属平台（1.淘宝、2.京东）
        stage: status_string, //拍卖阶段（一拍、二拍、变卖、撤回）
        photos: photos_vec,
    })
}
