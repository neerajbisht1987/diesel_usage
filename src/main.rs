
#![feature(plugin, const_fn, decl_macro, custom_attribute, proc_macro_hygiene)]
#![allow(proc_macro_derive_resolution_fallback, unused_attributes)]

#[macro_use]
extern crate diesel;

extern crate dotenv;

extern crate r2d2; 
extern crate r2d2_diesel; 
#[macro_use]
extern crate rocket;


#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

mod schema;
mod models;
mod static_files;
mod db;


fn rocket() ->rocket::Rocket {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let pool = db::init_pool(database_url);
    rocket::ignite()
    .manage(pool)
    .mount("/", routes![static_files::all, static_files::index])
   
}
fn main() {
     rocket().launch();
    }
