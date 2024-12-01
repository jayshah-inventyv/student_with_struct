pub mod routes;
use std::net::SocketAddr;
pub mod calculate_grade_percentage;
#[tokio::main]
async fn main() {
    let port=3000;
    let app=routes::get_routes();
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Started Server On Port {:?}",port);

    axum::Server::bind(&addr)
    .serve(app.clone().into_make_service())
    .await
    .expect("something went wrong!");
}
