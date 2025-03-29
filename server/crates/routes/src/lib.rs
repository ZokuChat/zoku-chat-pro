mod root;

use axum::{Router, routing::*};

pub fn build_routes() -> Router {
    Router::new().route("/", get(root::handler))
}
