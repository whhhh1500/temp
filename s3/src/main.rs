
use rusqlite::Connection;
use axum::routing::get;
use axum::Router;
use axum::extract::Query;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

// use std::path::Path;
#[allow(dead_code)]
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct User {
    id: u64,
    pwd: String,
    username: String,
    email: String,
    class: u64,
}


#[warn(unused_imports)]
#[tokio::main]
async fn main(){
   let   conn: Connection =Connection::open("data.db").unwrap();
    let _  =conn.execute("drop TABLE User",[]).unwrap();
    conn.execute(
        "CREATE TABLE User (
                id    INTEGER PRIMARY KEY,
                pwd  TEXT  ,
                username  TEXT,
                email  TEXT,
                class  INTEGER 
            )",
        (), // empty list of parameters.
    ).unwrap();
    let me = User {
        id: 0,
        pwd: "123".to_string(),
        username: "Steven".to_string(),
        email: "1".to_string(),
        class: 1,
    };
    conn.execute(
        "INSERT INTO User (pwd, username,email,class) VALUES (?1, ?2,?3,?4)",
        (&me.pwd, &me.username, &me.email, &me.class),
    ) .unwrap();
    // let mut stmt = conn.prepare("SELECT id, username, pwd,email,class FROM User")?;
    // let person_iter = stmt.query_map([], |row| {
    //     Ok(User {
    //         id: row.get(0)?,
    //         username: row.get(1)?,
    //         pwd: row.get(2)?,
    //         email:row.get(3)?,
    //         class:row.get(4)?
    //     })
    // })?;
    //
    // for user in person_iter {
    //     println!("Found user {:?}", user.unwrap());
    // }

    let app = Router::new()
        .route("/get", get(getOne))
        .route("/add", get(addOne));

    //0.0.0.0 表示所有ip都可以访问，3000是访问端口
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();


}
async fn getOne( Query(mut params): Query<HashMap<String, String>>) -> String {
    let id = params.remove("id").unwrap();
    println!("{:#?}", id);
    let   conn: Connection =Connection::open("data.db").unwrap();
    let mut stmt = conn.prepare(&("SELECT id  , username, pwd,email,class FROM User where id=".to_owned()+&id+"")).unwrap();
    let person_iter = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            pwd: row.get(2)?,
            email:row.get(3)?,
            class:row.get(4)?
        })

    }).unwrap();

    let mut  users =Vec::new();
    for u in person_iter{
        users.push(u.unwrap());
    }
    serde_json::to_string(&users).unwrap()
}
async fn addOne( Query(mut params): Query<HashMap<String, String>>) -> String{
    let   conn: Connection =Connection::open("data.db").unwrap();
    conn.execute(
        "INSERT INTO User (pwd, username,email,class) VALUES (?1, ?2,?3,?4)",
        ( params.remove("pwd").unwrap(), params.remove("username").unwrap(), params.remove("email").unwrap(), params.remove("class").unwrap()),
    ) .unwrap();
    "OK".to_string()
}