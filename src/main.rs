mod todos;

use axum::{
    error_handling::HandleErrorLayer,
    extract::{Extension},
    http::StatusCode,
    routing::{get, patch},
    Router,
};
use std::{
    net::SocketAddr,
    time::Duration,
};
use std::sync::Arc;
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use crate::todos::{DDBRepo, handlers::{todos_create, todos_delete, todos_index, todos_update}};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "todos=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db = Arc::new(DDBRepo::new());

    // Compose the routes
    let app = Router::new()
        .route("/", get(root))
        .route("/todos", get(todos_index).post(todos_create))
        .route("/todos/:id", patch(todos_update).delete(todos_delete))
        // Add middleware to all routes
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {}", error),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .layer(Extension(db))
                .into_inner(),
        );
    #[cfg(debug_assertions)]
    {
        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        tracing::debug!("listening on {}", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
    #[cfg(not(debug_assertions))]
    {
        // To run with AWS Lambda runtime, wrap in our `LambdaLayer`
        let app = tower::ServiceBuilder::new()
            .layer(axum_aws_lambda::LambdaLayer::default())
            .service(app);

        lambda_http::run(app).await.unwrap();
    }
}

// The query parameters for todos index

async fn root() -> &'static str {
    #[cfg(debug_assertions)]
    {
        "This is from local!"
    }
    #[cfg(not(debug_assertions))]
    {
        "This is from lambda!"
    }
}
















