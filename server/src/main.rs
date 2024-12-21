use warp::Filter;

#[tokio::main]
async fn main() {
    // Define the route for rqv1
    let route = warp::post()
        .and(warp::path("rqv1"))
        .map(|| {
            println!("handling");
            warp::reply::with_status("Handled", warp::http::StatusCode::OK)
        });

    // Start the server
    warp::serve(route).run(([127, 0, 0, 1], 3030)).await;
}
