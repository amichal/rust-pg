extern crate postgres;
extern crate iron;

use postgres::{Connection, SslMode};
use iron::prelude::*;
use iron::status;

struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>
}

fn main() {
    fn hello_world(_: &mut Request) -> IronResult<Response> {
        let mut s = String::new();
        pg(&mut s);
        Ok(Response::with((status::Ok, s)))
    }

    Iron::new(hello_world).http("localhost:3000").unwrap();
    println!("On 3000");
}

fn pg(output: &mut String) {
    let conn = Connection::connect(env!("DATABASE_URL"), &SslMode::None)
            .unwrap();

    conn.execute("DROP TABLE IF EXISTS person;", &[]).unwrap();

    conn.execute("CREATE TABLE person (
                    id              SERIAL PRIMARY KEY,
                    name            VARCHAR NOT NULL,
                    data            BYTEA
                  )", &[]).unwrap();
    output.push_str("table created...\n");
    let mut n = 0;
    for _ in 0..1 {
	    conn.execute("BEGIN TRANSACTION", &[]).unwrap();
	    for _ in 0..10000 {
	    	n += 1;
		    let me = Person {
		        id: 0,
		        name: format!("Steven {}", n),
		        data: None
		    };
		    conn.execute("INSERT INTO person (name, data) VALUES ($1, $2)",
		                 &[&me.name, &me.data]).unwrap();
		}
		conn.execute("COMMIT TRANSACTION", &[]).unwrap();
    	output.push_str(&format!("Done inserting {} people", n));
    }
    let stmt = conn.prepare("SELECT id, name, data FROM person where id < $1").unwrap();
    n = 0;
    for row in stmt.query(&[&1000]).unwrap() {
        let person = Person {
            id: row.get(0),
            name: row.get(1),
            data: row.get(2)
        };
        n += 1;
        output.push_str(&format!("Found {}", person.name));
    }
    output.push_str(&format!("Found {} people", n));

}