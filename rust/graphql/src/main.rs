mod schema;
use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use graphql::config::Config;
use juniper::http::{GraphQLRequest, GraphQLResponse, graphiql::graphiql_source};

use std::sync::Arc;

use crate::schema::Context;

#[tokio::main]
async fn main() {
    let config = Config::new();
    let pool = sqlx::sqlite::SqlitePool::connect(&config.db_url)
        .await
        .unwrap();

    let state = Arc::new(Context { pool });

    let router = Router::new()
        .route("/graphql", post(graphql))
        .route("/schema", get(sdl))
        .route("/", get(gui))
        .with_state(state);

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect(format!("Failed to listen at {}", addr).as_str());

    axum::serve(listener, router).await.unwrap();
}

async fn graphql(
    State(context): State<Arc<Context>>,
    Json(payload): Json<GraphQLRequest>,
) -> axum::response::Json<GraphQLResponse> {
    let schema = schema::create_schema();
    let res = payload.execute(&schema, &context).await;
    axum::response::Json(res)
}

async fn gui() -> axum::response::Html<String> {
    axum::response::Html(graphiql_source("/graphql", None))
}

async fn sdl() -> axum::response::Response {
    let schema = schema::create_schema();
    axum::response::Response::new(schema.as_sdl().into())
}
