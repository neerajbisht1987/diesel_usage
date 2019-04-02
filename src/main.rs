#[macro_use]
extern crate diesel;

extern crate dotenv;

use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

mod schema;
mod models;

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let conn= MysqlConnection::establish(&database_url).unwrap();

    let book = models::NewBook {
        title: String::from("Gravity Rainbow"),
        author:String::from("Thomas Pynchon"),
        published:true,
    };
    if models::Book::insert(book, &conn) {
        println!("success" );
    }
    else{
        println!("failed") 
    }
    let books = models::Book::all(&conn);
     for book in books
     {
         println!(" {},{}, {}, {}", book.id,book.title,book.author,book.published);
     }
     
    }
