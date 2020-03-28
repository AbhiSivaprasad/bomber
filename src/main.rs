use mongodb::{options::ClientOptions, Client};
use warp::Filter;

#[tokio::main]
async fn main() {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("bomber");

    // /GET / serves index.html
    let index_route = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("./static/index.html"));

    // /GET /static/* serves from ./static dir
    let static_res_route = warp::path("static").and(warp::fs::dir("./static/"));

    let routes = index_route.or(static_res_route);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
