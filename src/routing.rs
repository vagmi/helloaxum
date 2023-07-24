mod api;

use axum::{Router, routing::get, extract::Path};

use crate::state::AppState;

async fn greet() -> String {
    "Hello, That Conf!".to_string()
}

async fn greet_name(Path((name,age)): Path<(String, i32)>)-> String {
    format!("Happy {} birthday, {}!", age, name)
}

fn simple_router() -> Router<AppState> {
    Router::new()
            .route("/", get(greet))
            .route("/:name", get(greet_name))
}

pub fn create_router(app_state: AppState) -> Router {
    let todo_api_routes = api::create_router();
    let app = Router::new()
                .merge(simple_router())
                .nest("/api/v1/todos", todo_api_routes)
                .with_state(app_state);

    app
}

#[cfg(test)]
mod tests {
    use axum::{extract::Path, http::Request, body::{Body, self}, Router};
    use tower::ServiceExt;
    use super::*;

    #[tokio::test]
    async fn test_greet() {
        assert_eq!(super::greet().await, 
                   "Hello, That Conf!".to_string())
    }

    #[tokio::test]
    async fn test_greet_name() {
        let res = greet_name(Path(("Axum".to_string(),42))).await;
        assert_eq!(res, "Happy 42 birthday, Axum!");
    
    }

    #[tokio::test]
    async fn test_create_router() {
        let router = create_router(AppState::new().await);
        let resp = router.oneshot(
            Request::builder()
            .method(axum::http::method::Method::GET)
                    .uri("/")
                    .body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(resp.status(), 200);
        
    }
}
