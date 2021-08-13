#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;

extern crate serde;
extern crate rocket_cors;

mod schema;
mod domain;
mod models;

use rocket::request::Request;
use rocket_contrib::databases::diesel::PgConnection;
use rocket_contrib::json::Json;
use rocket_cors::{AllowedHeaders};
use rocket::http::Method;
use diesel::prelude::*;
use schema::*;
use models::*;

#[get("/")]
fn index(_db_conn: RustyDbConn) -> Json<Vec<Product>> {
    let results = products::table.filter(products::published.eq(false))
        .limit(5)
        .load::<Product>(&*_db_conn)
        .expect("Error loading posts");

    // Rocket uses the RustyDbConn request guard to provide us with a database
    // connection from a managed pool.
    Json(results)
}

#[catch(503)]
fn service_not_available(_req: &Request) -> &'static str {
    "Service is not available. (Is the database up?)"
}

#[database("rustydb")]
pub struct RustyDbConn(PgConnection);

fn main() {
    let cors = rocket_cors::CorsOptions {
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }.to_cors()
    .expect("error creating CORS fairing");

    rocket::ignite()
        .attach(RustyDbConn::fairing())
        .attach(cors)
        .register(catchers![service_not_available])
        .mount("/api", routes![index])
        .launch();
}
