use juniper::{FieldError, FieldResult, graphql_value};

use super::{Context, post::Post};

pub struct Mutations;

#[juniper::graphql_object(Context = Context)]
impl Mutations {
    async fn make_post(context: &Context, user_id: i32, content: String) -> FieldResult<Post> {
        let query = format!(
            "INSERT INTO Post (content, author_id) VALUES ('{content}', {user_id}) RETURNING *;"
        );

        sqlx::query_as(query.as_str())
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
