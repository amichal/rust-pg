/// quick hack at a building a traditional web application in rust

// library dependancies via Cargo crates

extern crate postgres;
extern crate iron;

// choose namespaces we need to use.
use std::io::prelude::*;
use std::io::{self};
use std::str::FromStr;

use std::env;

use iron::prelude::*;
use iron::status;

use postgres::{Connection, SslMode};


/// a structure mapping to a database row.
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>
}


/// main entry point.
fn main() {
    // setup a iron request handler (notice this is a inner function
    // of main)
    fn handler(_: &mut Request) -> IronResult<Response> {

        // we will need a mutable buffer to write to
        // Vec::new() gets is an expandable array of bytes
        // that supports the Write trate
        let mut buffer = Vec::new();

        // call our funtion with that to fill it up
        pg(&mut buffer);

        // return from our handler a Ok result (rust-wise)
        // containing a status::Ok http response with our
        // buffer as the body
        Ok(Response::with((status::Ok, buffer)))
    }

    // bind Iron to all ipv4 interfaces on some port.
    // in rust we call unwrap() a lot to consume the Result of
    // any functions that return a Result. this avoids warnings
    // and helps the compiler know when to free return memory
    Iron::new(handler).http(("0.0.0.0", get_server_port())).unwrap();
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

/// Get a postgres database URL.
#[test]
fn test_pg() {
  // this is cool. you can write tests in inline and 'feature flag' them to onlu get built in some modes. Include this function and run it with `cargo test`
	pg(&mut std::io::stdout());
}

/// Dumping groud and the main body of our code.
/// See the docs for rust-postgres https://github.com/sfackler/rust-postgres for the details of the API. Most of this is direct copy paste from there.
fn pg(output: &mut Write) {
    let mut n = 0;


    // connect to postgress
    let conn = Connection::connect(&*get_pg_url(), &SslMode::None)
            .unwrap();

  // table creation/population code. which i'm not running right now
  //   conn.execute("DROP TABLE IF EXISTS person;", &[]).unwrap();
  //   conn.execute("CREATE TABLE person (
  //                   id              SERIAL PRIMARY KEY,
  //                   name            VARCHAR NOT NULL,
  //                   data            BYTEA
  //                 )", &[]).unwrap();
  //   writeln!(output, "table created...");
  //   for _ in 0..1 {
	 //    conn.execute("BEGIN TRANSACTION", &[]).unwrap();
	 //    for _ in 0..100 {
	 //    	n += 1;
		//     let me = Person {
		//         id: 0,
		//         name: format!("Steven {}", n),
		//         data: None
		//     };
		//     conn.execute("INSERT INTO person (name, data) VALUES ($1, $2)",
		//                  &[&me.name, &me.data]).unwrap();
		// }
		// conn.execute("COMMIT TRANSACTION", &[]).unwrap();
  //   	writeln!(output, "Done inserting {} people", n);
  //   }

    // select prepared statement (which we bind to arguments below)
    let stmt = conn.prepare("SELECT id, name, data FROM person where id < $1").unwrap();
    n = 0;
    for row in stmt.query(&[&1000]).unwrap() {
        let person = Person {
            id: row.get(0),
            name: row.get(1),
            data: row.get(2)
        };
        n += 1;
        writeln!(output, "Found id={}, name={}", person.id, person.name);
    }
    writeln!(output, "Found {} people", n);
}