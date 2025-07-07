use juniper::{FieldError, FieldResult, graphql_value};

use super::{Context, post::Post, user::User};

pub struct Query;

#[juniper::graphql_object(Context = Context)]
impl Query {
    /// Greets the user
    fn hello() -> FieldResult<String> {
        Ok(String::from("Hello!"))
    }

    /// Greets the user with a name
    fn greeting(name: String) -> FieldResult<String> {
        Ok(format!("Hello, {name}"))
    }

    /// Gets a post by their username
    async fn get_user_by_name(context: &Context, username: String) -> FieldResult<User> {
        sqlx::query_as("SELECT * FROM User WHERE username=$1;")
            .bind(&username)
            .fetch_one(&context.pool)
            .await
            .map_err(|e| {
                FieldError::new(e.to_string(), graphql_value!({ "error": "User not found"}))
            })
    }

    /// Lists all posts
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
