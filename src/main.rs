use axum::Server;
use axum::Router;
use axum::routing::get;

async fn greet() -> String {
    "Hello, That Conf!".to_string()
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(greet));
    Server::bind(&"0.0.0.0:3333".parse().unwrap())
           .serve(app.into_make_service()).await.unwrap();
}
// cargo add tokio --features full
// cargo watch -c -q -x run
