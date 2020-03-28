use warp::Filter;

#[tokio::main]
async fn main() {
    // /GET / serves index.html
    let index_route = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("../client/dist/index.html"));

    // /GET /* serves from ../client/dist dir
    let static_res_route = warp::fs::dir("../client/dist");

    let routes = index_route.or(static_res_route);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
