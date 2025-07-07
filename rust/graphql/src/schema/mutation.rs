use juniper::{FieldError, FieldResult, graphql_value};

use super::{Context, post::Post};

pub struct Mutations;

#[juniper::graphql_object(Context = Context)]
impl Mutations {
    async fn make_post(context: &Context, user_id: i32, content: String) -> FieldResult<Post> {
        sqlx::query_as("INSERT INTO Post (content, author_id) VALUES ($1, $2) RETURNING *;")
            .bind(&content)
            .bind(&user_id)
            .fetch_one(&context.pool)
            .await
            .map_err(|e| {
                FieldError::new(
                    e.to_string(),
                    graphql_value!({ "error": "Failed to make post"}),
                )
            })
    }
}
