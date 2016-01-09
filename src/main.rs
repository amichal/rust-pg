/// quick hack at a building a traditional web application in rust

// library dependancies via Cargo crates

extern crate iron;
extern crate persistent;
extern crate logger;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate postgres;

use std::str::FromStr;
use std::io::prelude::*;
use std::env;
use iron::prelude::*;
use iron::typemap::Key;
use iron::mime::Mime;
use logger::Logger;


// Postgres crates
use r2d2::{Pool, PooledConnection};
use r2d2_postgres::{PostgresConnectionManager};

// Types

pub type PostgresPool = Pool<PostgresConnectionManager>;
pub type PostgresPooledConnection = PooledConnection<PostgresConnectionManager>;
pub struct AppDb;
impl Key for AppDb { type Value = PostgresPool; }


#[allow(dead_code)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>
}

/// main entry point.
fn main() {
    let conn_string = get_pg_url();
    println!("connecting to postgres: {}", conn_string);
    let pool = setup_connection_pool(&conn_string, 10);
    let (logger_before, logger_after) = Logger::new(None);

    let mut chain = Chain::new(basic_handler);
    chain.link(persistent::Read::<AppDb>::both(pool));
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    Iron::new(chain).http(("0.0.0.0", get_server_port())).unwrap();
}


// Helper methods
fn setup_connection_pool(cn_str: &str, pool_size: u32) -> PostgresPool {
    let manager = ::r2d2_postgres::PostgresConnectionManager::new(cn_str, ::r2d2_postgres::SslMode::None).unwrap();
    let config = ::r2d2::Config::builder().pool_size(pool_size).build();
    ::r2d2::Pool::new(config, manager).unwrap()
}

fn basic_handler(req: &mut Request) -> IronResult<Response> {
    let content_type = "text/html".parse::<Mime>().unwrap();
    let mut body = Vec::new();


    writeln!(body, "<link rel=\"icon\" type=\"image/png\" href=\"data:;base64,iVBORw0KGgo=\"><body><h1>Hello World</h1>").unwrap();

    let pool = req.get::<persistent::Read<AppDb>>().unwrap();
    let conn = pool.get().unwrap();
    let stmt = conn.prepare("SELECT id, name, data FROM person where id < $1").unwrap();
    let mut n = 0;
    for row in stmt.query(&[&1000]).unwrap().iter() {
        let person = Person {
            id: row.get(0),
            name: row.get(1),
            data: row.get(2)
        };
        n += 1;
        writeln!(body, "<p>Found id={}, name={}", person.id, person.name).unwrap();
    }
    writeln!(body, "<p>Found {} people", n).unwrap();

    Ok(
      Response::with((
        content_type,
        iron::status::Ok,
        body
      ))
    )
}

/// Look up our server port number in PORT env, for compatibility with Heroku.
fn get_server_port() -> u16 {

    // this is copy/paste from https://github.com/emk/heroku-rust-cargo-hello. Feels like there might be a more compact way to write it
    let port_str = env::var("PORT").unwrap_or(String::new());
    FromStr::from_str(&port_str).unwrap_or(3000)
}

/// Get the database url from the environment
fn get_pg_url() -> String {
	// pull a database url from the environment or default to localhost
  env::var("DATABASE_URL").unwrap_or("postgres://postgres@localhost".to_string())
}