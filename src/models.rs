use diesel::{deserialize::{Queryable, self}, prelude::Insertable, query_builder::AsChangeset};
use serde:: { Serialize, Deserialize };
use crate::schema::rustaceans;


#[derive(Serialize, Queryable, Deserialize, AsChangeset)]
pub struct Rustacean {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_deserializing)]
    pub create_at: String,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = rustaceans)]
pub struct  NewRustacean {
    pub name: String,
    pub email: String,
}