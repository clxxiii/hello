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
        sqlx::query_as("SELECT * FROM Post WHERE author_id=$1;")
            .bind(&self.id)
            .fetch_all(&context.pool)
            .await
            .unwrap_or(Vec::new())
    }
}
