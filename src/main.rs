mod routing;
mod state;

use axum::Server;
use routing::create_router;

#[tokio::main]
async fn main() {
    let app_state = state::AppState::new().await;
    let app = create_router(app_state);
    Server::bind(&"0.0.0.0:3333".parse().unwrap())
           .serve(app.into_make_service()).await.unwrap();
}
// cargo add tokio --features full
// cargo watch -c -q -x run
