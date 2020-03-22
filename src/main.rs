use warp::Filter;

#[tokio::main]
async fn main() {
    let route = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("./static/index.html"));

    warp::serve(route).run(([127, 0, 0, 1], 3030)).await;
}
