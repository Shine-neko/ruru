use warp::Filter;
use rusqlite::{params, Connection, Result};

#[tokio::main]
async fn main() {

    let connection = Connection::open_in_memory().unwrap();

    connection.execute(
        "CREATE TABLE person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  data            BLOB
                  )",
        [],
    ).unwrap();

    let list1 = warp::get()
        .and(warp::path("hello"))
        .map(|| {
            let mut stmt = connection.prepare("SELECT id, name, data FROM person").unwrap();
            format!("Hello!")
        });

    let list2 = warp::get()
        .and(warp::path("test"))
        .map(|| {
            format!("test!")
        });

    warp::serve(list1.or(list2))
        .run(([127, 0, 0, 1], 3030))
        .await;
}