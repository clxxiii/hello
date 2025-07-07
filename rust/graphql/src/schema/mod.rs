pub mod like;
pub mod post;
pub mod root;
pub mod user;

use juniper::RootNode;
use root::{MutationRoot, QueryRoot, SubscriptionRoot};
use sqlx::{Pool, Sqlite};

#[derive(Clone)]
pub struct Context {
    pub pool: Pool<Sqlite>,
}

impl juniper::Context for Context {}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, SubscriptionRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot, SubscriptionRoot)
}
