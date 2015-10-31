extern crate postgres;
extern crate iron;

use std::io::prelude::*;
use std::io::{self};
use std::str::FromStr;
use std::env;
use iron::prelude::*;
use iron::status;
use postgres::{Connection, SslMode};

struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>
}


fn main() {
    fn handler(_: &mut Request) -> IronResult<Response> {
        let mut buffer = Vec::new();
        pg(&mut buffer);
        Ok(Response::with((status::Ok, buffer)))
    }
    Iron::new(handler).http(("0.0.0.0", get_server_port())).unwrap();
}

/// Look up our server port number in PORT, for compatibility with Heroku.
fn get_server_port() -> u16 {
    let port_str = env::var("PORT").unwrap_or(String::new());
    FromStr::from_str(&port_str).unwrap_or(3000)
}

/// Get the database url from the environment
fn get_pg_url() -> String {
	env::var("DATABASE_URL").unwrap_or("postgres://postgres@localhost".to_string())
}

/// Get a postgres database URL.

#[test]
fn test_pg() {
	pg(&mut std::io::stdout());
}

fn pg(output: &mut Write) {
    let mut n = 0;
    let conn = Connection::connect(&*get_pg_url(), &SslMode::None)
            .unwrap();

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