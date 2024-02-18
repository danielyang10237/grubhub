use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

mod group;
mod group_search;
mod groups_all;
mod mark_as_read;
mod tags_all;
mod user_inbox;

pub fn router() -> Router {
    Router::new()
        .route("/groups/:groupid", get(group::handle))
        .route("/groups/search", post(group_search::handle))
        .route("/groups/all", get(groups_all::handle))
        .route("/users/:userid/inbox", get(user_inbox::handle))
        .route("/tags/all", get(tags_all::handle))
        .route("/users/mark_as_read", post(mark_as_read::handle))
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST])
                .allow_origin(Any),
        )
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST])
                .allow_origin(Any)
                .allow_headers(Any),
        )
}
