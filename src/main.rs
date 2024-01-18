#[macro_use] extern crate rocket;

mod auth;
mod schema;
mod models;

use diesel::prelude::*;
use auth::BasicAuth;
use rocket::serde::json::{Value, json, Json};
use rocket::response::status;
use rocket_sync_db_pools::database;
use schema::rustaceans;
use models::{ Rustacean, NewRustaceans };


#[database("sqlite")]

struct DbConn(diesel::SqliteConnection);

#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, db: DbConn) -> Value{
    db.run(|c| {
        let rustaceans = rustaceans::table.order(rustaceans::id.desc()).limit(1000).load::<Rustacean>(c).expect("DB error");
        json!(rustaceans)
    }).await
}
#[get("/rustaceans/<id>")]
fn view_rustacean(id: i32, _auth: BasicAuth) -> Value{
    json!([{"id": id, "name": "John Doe", "email": "john@doe.com" }])
}
#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn create_rustacean(_auth: BasicAuth, db: DbConn, new_rustacean: Json<NewRustaceans>) -> Value{
    db.run(|c| {
       let result = diesel::insert_into(rustaceans::table).values(new_rustacean.into_inner()).execute(c).expect("DB error when inserting");
       json!(result)
    }).await
}
#[put("/rustaceans/<id>")]
fn update_rustacean(id: i32, _auth: BasicAuth) -> Value{
    json!([{"id": id, "name": "John Doe", "email": "john@doe.com" }])
}
#[delete("/rustaceans/<id>")]
fn delete_rustacean(id: i32, _auth: BasicAuth) -> status::NoContent {
    status::NoContent
}
#[catch(422)]
fn unprocessable_content() -> Value {
    json!("Unprocessable Content!")
}

#[catch(404)]
fn not_found() -> Value {
    json!("Not found!")
}

#[catch(401)]
fn not_authorised() -> Value {
    json!("Not Authorised")
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![
            get_rustaceans,
            view_rustacean,
            create_rustacean,
            update_rustacean,
            delete_rustacean
        ])
        .register("/", catchers![
            not_found,
            not_authorised,
            unprocessable_content
        ])
        .attach(DbConn::fairing())
        .launch()
        .await;
}
