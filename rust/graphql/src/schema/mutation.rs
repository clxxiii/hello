use std::sync::Arc;

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
    async fn like_post(context: &Context, post_id: i32) -> FieldResult<Arc<Post>> {
        let post: Post =
            sqlx::query_as("UPDATE Post SET likes = likes + 1 WHERE id=$1 RETURNING *;")
                .bind(&post_id)
                .fetch_one(&context.pool)
                .await
                .map_err(|e| {
                    FieldError::new(
                        e.to_string(),
                        graphql_value!({ "error": "failed to update post"}),
                    )
                })?;

        let post = Arc::new(post);
        let _ = context.post_likes_broadcast.send(Arc::clone(&post));

        Ok(post)
    }
}
