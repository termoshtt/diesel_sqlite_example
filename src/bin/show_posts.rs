
extern crate diesel_sqlite_sample;
extern crate diesel;

use diesel_sqlite_sample::*;
use diesel_sqlite_sample::models::*;
use diesel::prelude::*;

fn main() {
    use diesel_sqlite_sample::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts.filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}
