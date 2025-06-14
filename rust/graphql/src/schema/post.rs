use crate::schema::user::User;

use super::Context;

#[derive(sqlx::FromRow)]
pub struct Post {
    id: i32,
    content: String,
    author_id: i32,
}

#[juniper::graphql_object(Context = Context)]
impl Post {
    fn id(&self) -> &i32 {
        &self.id
    }
    fn content(&self) -> &String {
        &self.content
    }
    async fn author(&self, context: &Context) -> Option<User> {
        let mut connection = context.pool.acquire().await.unwrap();

        let query = format!("SELECT * FROM User WHERE id={}", self.author_id);
        match sqlx::query_as(query.as_str())
            .fetch_one(connection.as_mut())
            .await
        {
            Ok(user) => Some(user),
            Err(_) => None,
        }
    }
}
