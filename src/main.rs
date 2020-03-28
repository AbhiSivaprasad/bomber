use mongodb::{options::ClientOptions, Client};
use std::sync::Arc;
use warp::Filter;

mod api;
mod user;

use api::*;

#[tokio::main]
async fn main() {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = Arc::new(client.database("mvp"));
    let db = warp::any().map(move || Arc::clone(&db));

    // /GET / serves index.html
    let index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("./static/index.html"));

    // /GET /static/* serves from ./static dir
    let static_res = warp::path("static").and(warp::fs::dir("./static/"));

    let register = warp::post()
        .and(warp::path("register"))
        .and(warp::body::json())
        .and(db.clone())
        .and_then(register);
    let login = warp::post()
        .and(warp::path("login"))
        .and(warp::body::json())
        .and(db.clone())
        .and_then(login);
    let logout = warp::post().and(warp::path("logout")).and_then(logout);

    let api = register.or(login).or(logout);
    let api = warp::path("api").and(api);

    let routes = index.or(static_res).or(api);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
