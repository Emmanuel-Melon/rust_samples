/**
 * A simple program to demonstrate connecting to different databases (MySQL, SQLite & PostgreSQL)
 */

#[macro_use]
extern crate diesel;
extern crate dotenv;

use dotenv::dotenv;
use std::env;

// diesel
use diesel::{mysql::MysqlConnection, pg::PgConnection, prelude::*};

// connect to PostgreSQL
fn connect_pg() -> PgConnection {
    dotenv().ok();
    let pg_url = env::var("DATABASE_URL").expect("POSTGRESQL_URL must be set");
    PgConnection::establish(&pg_url).expect(&format!("Error connecting to {}", pg_url))
}

// connect to MySQL
fn connect_mysql() -> MysqlConnection {
    dotenv().ok();
    let mysql_url = env::var("DATABASE_URL").expect("MYSQL_URL must be set");
    MysqlConnection::establish(&mysql_url).expect(&format!("Error connecting to {}", mysql_url))
}

/**
 * models
 */
#[derive(Queryable)] // assumes same field order as schema
struct User {
    pub userid: i32,
    pub username: String,
    pub emaily: String,
}

fn main() {
    // bring dsl into scope

    let pg_connection = connect_pg();
    let results = users
        .filter(active.eq(true))
        .limit(5)
        .load::<User>(&pg_connection)
        .expect("Failed to load users");

    println!("Displaying {} posts", results.len());
    for user in results {
        println!("{}", user.username);
        println!("----------\n");
        println!("{}", user.email);
    }
}
