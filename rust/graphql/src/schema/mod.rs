pub mod like;
pub mod post;
pub mod root;
pub mod user;

use juniper::{EmptySubscription, RootNode};
use root::QueryRoot;
use sqlx::{Pool, Sqlite};

use crate::schema::root::MutationRoot;

pub struct Context {
    pub pool: Pool<Sqlite>,
}

impl juniper::Context for Context {}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot, EmptySubscription::new())
}
