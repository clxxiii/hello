use std::time::Duration;

use super::{Context, post::Post, user::User};
use futures::stream::{BoxStream, StreamExt as _};
use juniper::{FieldError, FieldResult, graphql_value};
use tokio::time::interval;
use tokio_stream::wrappers::IntervalStream;

pub struct Query;

#[juniper::graphql_object(Context = Context)]
impl Query {
    #[graphql(description = "Greets the user")]
    fn hello() -> FieldResult<String> {
        Ok(String::from("Hello!"))
    }

    #[graphql(description = "Greets the user with a name")]
    fn greeting(name: String) -> FieldResult<String> {
        Ok(format!("Hello, {name}"))
    }

    #[graphql(description = "Gets a post by their username")]
    async fn get_user_by_name(context: &Context, username: String) -> FieldResult<User> {
        let query = format!("SELECT * FROM User WHERE username='{username}';");
        sqlx::query_as(query.as_str())
            .fetch_one(&context.pool)
            .await
            .map_err(|e| {
                FieldError::new(e.to_string(), graphql_value!({ "error": "User not found"}))
            })
    }

    #[graphql(description = "Lists all posts")]
    async fn all_posts(context: &Context) -> FieldResult<Vec<Post>> {
        sqlx::query_as("SELECT * FROM Post;")
            .fetch_all(&context.pool)
            .await
            .map_err(|_| {
                FieldError::new(
                    "Failed to fetch post",
                    graphql_value!({ "error": "Failed to fetch posts"}),
                )
            })
    }
}

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

pub struct Subscriptions;

type NumberStream = BoxStream<'static, FieldResult<i32>>;

#[juniper::graphql_subscription(Context = Context)]
impl Subscriptions {
    /// Counts seconds.
    async fn count() -> NumberStream {
        let mut value = 0;
        let stream = IntervalStream::new(interval(Duration::from_secs(1))).map(move |_| {
            value += 1;
            Ok(value)
        });
        Box::pin(stream)
    }
}
