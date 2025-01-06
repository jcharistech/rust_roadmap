use sqlx::{postgres::PgPoolOptions, PgPool};


// Create DB Fxn for connection
pub async fn establish_connection() -> Result<PgPool, sqlx::Error> {
    let database_url = "postgres://postgres:password123@localhost:5432/rustdb";
    PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await
}
