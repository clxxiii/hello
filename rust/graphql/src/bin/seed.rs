use std::fs;

use graphql::config::Config;

#[tokio::main]
async fn main() {
    let config = Config::new();
    let pool = sqlx::sqlite::SqlitePool::connect(&config.db_url)
        .await
        .expect("Failed to open the database");

    let migration = fs::read_to_string("seed-migration.sql").unwrap();
    match sqlx::query(&migration).fetch_optional(&pool).await {
        Ok(_) => println!("Successfully seeded the database!"),
        Err(e) => println!("{e}"),
    }
}
