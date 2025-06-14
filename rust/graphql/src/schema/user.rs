use crate::schema::post::Post;

use super::Context;

#[derive(sqlx::FromRow)]
pub struct User {
    id: i32,
    username: String,
}

#[juniper::graphql_object(Context = Context)]
impl User {
    pub fn id(&self) -> &i32 {
        &self.id
    }

    pub fn username(&self) -> &String {
        &self.username
    }

    async fn posts(&self, context: &Context) -> Vec<Post> {
        let mut connection = context.pool.acquire().await.unwrap();

        let query = format!("SELECT * FROM Post WHERE author_id={}", self.id);
        match sqlx::query_as(query.as_str())
            .fetch_all(connection.as_mut())
            .await
        {
            Ok(posts) => posts,
            Err(_) => Vec::new(),
        }
    }
}
