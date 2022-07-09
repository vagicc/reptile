use crate::db::get_connection;
use crate::schema::lawsuit_reptile;
use crate::schema::lawsuit_reptile::dsl::*;
use chrono::NaiveDateTime;
use diesel::data_types::Cents;  //i64 单位为分
use diesel::prelude::*;

// 新插入数据结构体
#[derive(Debug, Clone, Insertable)]
#[table_name = "lawsuit_reptile"]
pub struct NewLawsuitReptile {
    pub title: String,
    pub price_base: Cents,    //起拍价
    pub current_price: Cents, //当前价
    pub assess_price: Cents,  //评估价
    pub margin: Cents,        //保证金
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub address: Option<String>,       //标地物详细地址
    pub disposal_unit: Option<String>, //处置单位:所属法院
    pub external_url: Option<String>,
    pub belong: Option<i16>,   //所属平台（1.淘宝、2.京东）
    pub stage: Option<String>, //拍卖阶段（一拍、二拍、变卖、撤回）
    pub status: i16,           //状态（1待开拍、2竞拍中、已结束:3成交，4流拍、0无效或撤回）
    pub create_time: Option<NaiveDateTime>,
}
impl NewLawsuitReptile {
    pub fn insert(&self) -> i32 {
        /* 处理创建时间 */
        // if self.create_time.is_none() {
        //     let now_date_time = crate::common::now_naive_date_time();
        //     self.create_time = Some(now_date_time);
        // }

        let connection = get_connection();
        let insert_id = diesel::insert_into(lawsuit_reptile)
            .values(self)
            .returning(id)
            .get_result::<i32>(&connection)
            .unwrap_or(0);
        insert_id
    }
}
