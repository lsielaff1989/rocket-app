#[macro_use] extern crate rocket;

mod auth;
mod schema;
mod models;
mod repositorys;


use auth::BasicAuth;
use diesel_migrations::EmbeddedMigrations;
use rocket::{Rocket, Build};
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::serde::json::{Value, json, Json};
use rocket::response::status::{self, Custom};
use rocket_sync_db_pools::database;
use diesel::result::Error::NotFound;
use models::{ Rustacean, NewRustacean };
use repositorys::RustaceanRepository;





#[database("sqlite")]

struct DbConn(diesel::SqliteConnection);

#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, db: DbConn) -> Result<Value, Custom<Value>>{
    db.run(|c| {
        RustaceanRepository::find_multiple(c, 100).map(|rustaceans| json!(rustaceans) ).map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
        
    }).await
}
#[get("/rustaceans/<id>")]
async fn view_rustacean(id: i32, _auth: BasicAuth, db: DbConn) -> Result<Value, Custom<Value>>{
    db.run(move |c| {
       RustaceanRepository::find(c, id)
       .map(|rustaceans| json!(rustaceans) )
       .map_err(|e|
            match e {
                NotFound => Custom(Status::NotFound, json!(e.to_string())),
                _ => Custom(Status::InternalServerError, json!(e.to_string()))
        } 
    )
       
    }).await
}
#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn create_rustacean(_auth: BasicAuth, db: DbConn, new_rustacean: Json<NewRustacean>) -> Result<Value, Custom<Value>>{
    db.run(|c| {
       RustaceanRepository::create(c, new_rustacean.into_inner()).map(|rustaceans| json!(rustaceans) ).map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
     
    }).await
}
#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
async fn update_rustacean(id: i32, _auth: BasicAuth, db: DbConn, rustacean: Json<Rustacean>) -> Result<Value, Custom<Value>>{
    db.run(move |c| {
        RustaceanRepository::save(c, id, rustacean.into_inner()).map(|rustaceans| json!(rustaceans) ).map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
        
    }).await
}
#[delete("/rustaceans/<id>")]
async fn delete_rustacean(id: i32, _auth: BasicAuth, db: DbConn) -> Result<status::NoContent, Custom<Value>> {
    db.run(move |c| {
            RustaceanRepository::delete(c, id).map(|_| status::NoContent ).map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
        }
      
        
    ).await
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

async fn run_db_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    use diesel_migrations::{embed_migrations, EmbeddedMigration, MigrationHarness};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
    DbConn::get_one(&rocket).await.expect("Unable to retrive connection").run(|c| {
        c.run_pending_migrations(MIGRATIONS).expect("Migrations Faild");
    }
    ).await;
    rocket
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
        .attach(AdHoc::on_ignite("Diesel migrations", run_db_migrations))
        .launch()
        .await;
}
