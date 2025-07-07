pub mod like;
pub mod post;
pub mod root;
pub mod user;

use juniper::RootNode;
use root::{Mutations, Query, Subscriptions};
use sqlx::{Pool, Sqlite};

#[derive(Clone)]
pub struct Context {
    pub pool: Pool<Sqlite>,
}

impl juniper::Context for Context {}

pub type Schema = RootNode<'static, Query, Mutations, Subscriptions>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutations, Subscriptions)
}
