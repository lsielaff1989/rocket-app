use diesel::{deserialize::{Queryable, self}, prelude::Insertable};
use serde:: { Serialize, Deserialize };
use crate::schema::rustaceans;


#[derive(Serialize, Queryable)]
pub struct Rustacean {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub create_at: String,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = rustaceans)]
pub struct  NewRustaceans {
    pub name: String,
    pub email: String,
}