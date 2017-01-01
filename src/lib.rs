#![cfg_attr(feature = "nightly", feature(proc_macro))]

#[macro_use]
extern crate diesel;
#[cfg(feature = "nightly")]
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;

#[cfg(feature = "nightly")]
include!("lib.in.rs");

#[cfg(feature = "with-syntex")]
include!(concat!(env!("OUT_DIR"), "/lib.rs"));

use std::env;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;

use self::schema::posts;
use self::models::{Post, NewPost};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_post(conn: &SqliteConnection, title: &str, body: &str) -> Post {
    let new_post = NewPost {
        title: title,
        body: body,
    };
    diesel::insert(&new_post)
        .into(posts::table)
        .get_result(conn)
        .expect("Error saving new post")
}
