#![feature(proc_macro_hygiene, decl_macro)]

use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[macro_use]
extern crate rocket;

#[derive(Serialize, Deserialize)]
struct TodoList {
    items: Vec<TodoItem>,
}
#[derive(Serialize, Deserialize)]
struct TodoItem {
    id: i64,
    item: String,
}

#[derive(Serialize)]
struct StatusMsg {
    message: String,
}

#[get("/")]
fn index() -> &'static str {
    "hello world form rocket, how are you"
}

#[get("/todos")]
fn fetch_all_todo_items() -> Result<Json<TodoList>, String> {
    let db_conn = match rusqlite::Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => return Err("Feild to connect to db".to_string()),
    };

    let mut query = match db_conn.prepare("select id, item from todo_list;") {
        Ok(query) => query,
        Err(_) => return Err("problem with query".into()),
    };

    let results = query.query_map([], |row| {
        Ok(TodoItem {
            id: row.get(0)?,
            item: row.get(1)?,
        })
    });

    match results {
        Ok(rows) => {
            let collection: rusqlite::Result<Vec<_>> = rows.collect();

            match collection {
                Ok(items) => Ok(Json(TodoList { items })),
                Err(_) => Err("could not collect items".into()),
            }
        }
        Err(_) => Err("feild to fetch TodoItems".into()),
    }
}

#[delete("/todo/<id>")]
fn delete_todo_item(id: i64) -> Result<Json<StatusMsg>, String> {
    let db_conn = match rusqlite::Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => return Err("could not open db".into()),
    };

    let mut query = match db_conn.prepare("delete from todo_list where id = $1;") {
        Ok(query) => query,
        Err(_) => return Err("could not delete data".into()),
    };

    let results = query.execute(&[&id]);

    match results {
        Ok(rows_affected) => Ok(Json(StatusMsg {
            message: format!("{} rows deleted", rows_affected),
        })),
        Err(_) => Err("Feild to insert todo items into todo_list db".into()),
    }
}

#[post("/todo", format = "json", data = "<item>")]
fn add_todo_item(item: Json<String>) -> Result<Json<StatusMsg>, String> {
    let db_conn = match rusqlite::Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => return Err("could not open db".into()),
    };

    let mut query = match db_conn.prepare("insert into todo_list (id, item) values (null, $1)") {
        Ok(query) => query,
        Err(_) => return Err("could not insert data".into()),
    };

    let results = query.execute(&[&item.0]);

    match results {
        Ok(rows_affected) => Ok(Json(StatusMsg {
            message: format!("{} rows inserted", rows_affected),
        })),
        Err(_) => Err("Feild to insert todo items into todo_list db".into()),
    }
}

fn main() {
    {
        let db_conn = rusqlite::Connection::open("data.sqlite").expect("Cannot open database");

        db_conn
            .execute(
                "create table if not exists todo_list (
                id integer primary key,
                item varchar(64) not null
            )",
                [],
            )
            .expect("Cannot create table todo_list");
    }
    rocket::ignite()
        .mount(
            "/",
            routes![index, fetch_all_todo_items, add_todo_item, delete_todo_item],
        )
        .launch();
}
