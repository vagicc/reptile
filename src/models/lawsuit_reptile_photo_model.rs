use crate::db::get_connection;
use crate::schema::lawsuit_reptile_photo;
use crate::schema::lawsuit_reptile_photo::dsl::*;
use diesel::prelude::*;

// 新插入数据结构体
#[derive(Debug, Clone, Insertable)]
#[table_name = "lawsuit_reptile_photo"]
pub struct NewLawsuitReptilePhoto {
    pub lrid: i32,
    pub external_small: Option<String>,
    pub external_middle: Option<String>,
    pub external_original: Option<String>,
    pub front_cover: Option<bool>,
}
impl NewLawsuitReptilePhoto {
    pub fn insert(&self) -> i32 {
        let connection = get_connection();
        let insert_id = diesel::insert_into(lawsuit_reptile_photo)
            .values(self)
            .returning(lrid)
            .get_result::<i32>(&connection)
            .unwrap_or(0);
        insert_id
    }
}
