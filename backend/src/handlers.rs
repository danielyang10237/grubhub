use axum::{
    routing::{get, post},
    Router,
};

mod group;
mod group_search;
mod groups_all;
mod tags_all;
mod user_inbox;

pub fn router() -> Router {
    Router::new()
        .route("/groups/:groupid", get(group::handle))
        .route("/groups/search", post(group_search::handle))
        .route("/groups/all", get(groups_all::handle))
        .route("/users/:userid/inbox", get(user_inbox::handle))
        .route("/tags/all", get(tags_all::handle))
}
