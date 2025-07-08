use crate::schema::user::User;

use super::Context;

#[derive(sqlx::FromRow)]
pub struct Post {
    id: i32,
    content: String,
    author_id: i32,
    likes: i32,
}

#[juniper::graphql_object(Context = Context)]
impl Post {
    fn id(&self) -> &i32 {
        &self.id
    }
    fn content(&self) -> &String {
        &self.content
    }
    fn likes(&self) -> &i32 {
        &self.likes
    }
    async fn author(&self, context: &Context) -> Option<User> {
        sqlx::query_as("SELECT * FROM User WHERE id=$1;")
            .bind(&self.author_id)
            .fetch_one(&context.pool)
            .await
            .ok()
    }
}
