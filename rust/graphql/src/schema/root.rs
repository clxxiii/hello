use super::{Context, post::Post, user::User};
use juniper::{FieldError, FieldResult, graphql_value};

pub struct QueryRoot;

#[juniper::graphql_object(Context = Context)]
impl QueryRoot {
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
        let mut connection = context.pool.acquire().await.unwrap();

        let query = format!("SELECT * FROM User WHERE username='{username}';");
        sqlx::query_as(query.as_str())
            .fetch_one(connection.as_mut())
            .await
            .map_err(|e| {
                FieldError::new(e.to_string(), graphql_value!({ "error": "User not found"}))
            })
    }

    #[graphql(description = "Lists all posts")]
    async fn all_posts(context: &Context) -> FieldResult<Vec<Post>> {
        let mut connection = context.pool.acquire().await.unwrap();

        sqlx::query_as("SELECT * FROM Post;")
            .fetch_all(connection.as_mut())
            .await
            .map_err(|_| {
                FieldError::new(
                    "Failed to fetch post",
                    graphql_value!({ "error": "Failed to fetch posts"}),
                )
            })
    }
}

pub struct MutationRoot;

#[juniper::graphql_object(Context = Context)]
impl MutationRoot {
    async fn make_post(context: &Context, user_id: i32, content: String) -> FieldResult<Post> {
        let mut connection = context.pool.acquire().await.unwrap();

        let query = format!(
            "INSERT INTO Post (content, author_id) VALUES ('{content}', {user_id}) RETURNING *;"
        );

        sqlx::query_as(query.as_str())
            .fetch_one(connection.as_mut())
            .await
            .map_err(|e| {
                FieldError::new(
                    e.to_string(),
                    graphql_value!({ "error": "Failed to make post"}),
                )
            })
    }
}
