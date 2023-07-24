use axum::{Router, routing::get, extract::Path};

async fn greet() -> String {
    "Hello, That Conf!".to_string()
}

async fn greet_name(Path((name,age)): Path<(String, i32)>)-> String {
    format!("Happy {} birthday, {}!", age, name)
}

pub fn create_router() -> Router {
    let app = Router::new()
                .route("/", get(greet))
                .route("/:name/:age", get(greet_name));
    app
}

#[cfg(test)]
mod tests {
    use axum::extract::Path;
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
}
