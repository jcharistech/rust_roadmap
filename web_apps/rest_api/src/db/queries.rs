
use sqlx::PgPool;
use crate::db::models::{BlogPost,NewBlogPost};

// SQLx Functions
pub async fn create_post(pool: &PgPool, post: &NewBlogPost) -> Result<BlogPost,sqlx::Error> {
    let post = sqlx::query_as::<_, BlogPost>( 
        "INSERT INTO blog_posts (title, content, author) VALUES ($1, $2, $3) RETURNING *",
).bind(&post.title)
.bind(&post.content)
.bind(&post.author)
.fetch_one(pool)
.await?;
Ok(post)
}


pub async fn get_posts(pool: &PgPool) -> Result<Vec<BlogPost>, sqlx::Error> {
    sqlx::query_as::<_, BlogPost>("SELECT * FROM blog_posts")
        .fetch_all(pool)
        .await
}

pub async fn get_post(pool: &PgPool, id: i32) -> Result<BlogPost, sqlx::Error> {
    sqlx::query_as::<_, BlogPost>("SELECT * FROM blog_posts WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn update_post(pool: &PgPool, id: i32, post: &BlogPost) -> Result<(), sqlx::Error> {
    let q = "UPDATE blog_posts SET title = $1, content = $2, author = $3 WHERE id = $4";
    sqlx::query(q)
    .bind(&post.title)
    .bind(&post.content)
    .bind(&post.author)
    .bind(&id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_post(pool: &PgPool, id: i32) -> Result<(), sqlx::Error> {
    let q = "DELETE FROM blog_posts WHERE id = $1";
    sqlx::query(q)
    .bind(&id)
    .execute(pool)
    .await?;
    Ok(())
}

