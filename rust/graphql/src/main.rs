mod schema;
use axum::{
    Router,
    extract::Extension,
    routing::{MethodFilter, get, on},
};
use graphql::config::Config;
use juniper_axum::{
    extract::JuniperRequest, graphiql, graphql_transport_ws, response::JuniperResponse,
};
use juniper_graphql_ws::ConnectionConfig;

use std::sync::Arc;

use schema::{Context, Schema};

async fn sdl(Extension(schema): Extension<Arc<Schema>>) -> axum::response::Response {
    axum::response::Response::new(schema.as_sdl().into())
}

async fn handle_graphql(
    Extension(schema): Extension<Arc<Schema>>,
    Extension(context): Extension<Context>,
    JuniperRequest(request): JuniperRequest,
) -> JuniperResponse {
    JuniperResponse(request.execute(&*schema, &context).await)
}

#[tokio::main]
async fn main() {
    let config = Config::new();
    let pool = sqlx::sqlite::SqlitePool::connect(&config.db_url)
        .await
        .unwrap();
    let schema = schema::create_schema();

    let state = Context { pool };

    let router = Router::new()
        .route(
            "/graphql",
            on(MethodFilter::GET.or(MethodFilter::POST), handle_graphql),
        )
        .route(
            "/subscriptions",
            get(graphql_transport_ws::<Arc<Schema>>(ConnectionConfig::new(
                state.clone(),
            ))),
        )
        .route("/schema", get(sdl))
        .route("/", get(graphiql("/graphql", "/subscriptions")))
        .layer(Extension(Arc::new(schema)))
        .layer(Extension(state));

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.expect(
        format!(
            "Address {} should be available so a connection can be opened",
            addr
        )
        .as_str(),
    );

    println!("Opened connection at http://localhost:3000");
    axum::serve(listener, router).await.unwrap();
}
