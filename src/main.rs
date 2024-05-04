mod pg;
use crate::pg::connection::{PgCon, ConnectionInfo, Address};

fn main() {
//    PgCon::new(&ConnectionInfo{addr:"127.0.0.1:5432", db: String::from("test")});
    PgCon::new(&ConnectionInfo{addr:Address::Socket("/run/postgresql/.s.PGSQL.5432".to_string()), db: String::from("test_db")});

    println!("Hello, world!");
}
