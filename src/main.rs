#[macro_use] extern crate rocket;

mod repositories;
mod models;
mod schema;

use rocket::{catch, catchers, delete, get, post, put, routes, Rocket, Build};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket_sync_db_pools::database;
use serde_json::{json, Value};
use crate::models::{NewUser, User};
use crate::repositories::UserRepository;
use diesel::result::Error::NotFound;
use rocket::fairing::AdHoc;
use rocket::response::status;


#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);

#[get("/users")]
async fn get_users(db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        UserRepository::get_all(c)
            .map(|users| json!(users))
            .map_err(|err|
                match err {
                    NotFound => Custom(Status::NotFound, json!({"status": "error", "reason": "Resource was not found."})),
                    _ => Custom(Status::InternalServerError, json!({"status": "error", "reason": err.to_string()}))
                }
            )
    }).await
}

#[get("/users/<id>")]
async fn get_user(id: i32, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        UserRepository::get_by_id(c, id)
            .map(|user| json!(user))
            .map_err(|err|
                Custom(Status::InternalServerError, json!({"status": "error", "reason": err.to_string()})))
    }).await
}

#[post("/users", format = "json", data = "<new_user>")]
async fn create_user(db:DbConn, new_user: Json<NewUser>) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        UserRepository::create(c, new_user.into_inner())
            .map(|user| json!(user))
            .map_err(|err|
                Custom(Status::InternalServerError, json!({"status": "error", "reason": err.to_string()})))
    }).await
}

#[put("/users", format = "json", data="<user>")]
async fn update_user(db: DbConn, user: Json<User>) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        UserRepository::update(c, user.into_inner())
            .map(|user| json!(user))
            .map_err(|err|
                Custom(Status::InternalServerError, json!({"status": "error", "reason": err.to_string()})))
    }).await
}

#[delete("/users/<id>")]
async fn delete_user(id: i32, db:DbConn) -> Result<status::NoContent, Custom<Value>> {
    db.run(move |c| {
        UserRepository::delete(c, id)
            .map(|_| status::NoContent)
            .map_err(|err|
                Custom(Status::InternalServerError, json!({"status": "error", "reason": err.to_string()})))
    }).await
}

#[catch(404)]
fn not_found() -> Value {
    json!({"status": "error", "reason": "Resource was not found."})
}

#[catch(401)]
fn unauthorized() -> Value {
    json!({"status": "error", "reason": "Unauthorized."})
}

#[catch(422)]
fn unprocessable_entity() -> Value {
    json!({"status": "error", "reason": "Unprocessable entity."})
}

async fn run_db_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

    DbConn::get_one(&rocket)
        .await
        .expect("Unable to retrieve connection").run(|c| {
            c.run_pending_migrations(MIGRATIONS).expect("Migrations failed");
        })
        .await;

    rocket
}


#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![
            get_users,
            get_user,
            create_user,
            update_user,
            delete_user
        ])
        .register("/", catchers![
            not_found,
            unauthorized,
            unprocessable_entity
        ])
        .attach(DbConn::fairing())
        .attach(AdHoc::on_ignite("Diesel migrations", run_db_migrations))
        .launch()
        .await;
}
