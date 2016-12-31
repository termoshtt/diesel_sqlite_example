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

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
