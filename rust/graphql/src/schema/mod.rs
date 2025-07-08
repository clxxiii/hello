pub mod like;
pub mod post;
pub mod user;

pub mod mutation;
pub mod query;
pub mod subscription;

use std::sync::Arc;

use tokio::sync::broadcast;

use juniper::RootNode;
use sqlx::{Pool, Sqlite};

use crate::schema::post::Post;

#[derive(Clone)]
pub struct Context {
    pub pool: Pool<Sqlite>,
    pub post_likes_broadcast: broadcast::Sender<Arc<Post>>,
}

impl juniper::Context for Context {}

pub type Schema = RootNode<'static, query::Query, mutation::Mutations, subscription::Subscriptions>;

pub fn create_schema() -> Schema {
    Schema::new(
        query::Query,
        mutation::Mutations,
        subscription::Subscriptions,
    )
}
