
use actix_web::{web, Responder,HttpResponse,post,get,put,delete};
use sqlx::PgPool;
use crate::db::models::{BlogPost,NewBlogPost};
use crate::db::queries;
use crate::api::errors::ApiError;


#[utoipa::path(
    post,
    path="/blog",
    request_body=NewBlogPost,
    responses(
        (status = 201,description="Post created successfully")
    )
)]
#[post("/blog")]
async fn create_blogpost(data: web::Data<PgPool>,new_post: web::Json<NewBlogPost>) -> Result<impl Responder, ApiError> {
    let post = queries::create_post(&data, &new_post).await.expect("Failed");
    Ok(HttpResponse::Ok().json(post))
}


#[utoipa::path(
    get,
    path="/blog",
    responses(
        (status = 200, body=Vec<BlogPost>,description="List of All Blog Posts")
    )
)]
#[get("/blog")]
async fn get_all_blogposts(data: web::Data<PgPool>) -> Result<impl Responder, ApiError> {
    let posts = queries::get_posts(&data).await.map_err(ApiError::from)?;
    Ok(HttpResponse::Ok().json(posts))
}



#[utoipa::path(
    get,
    path = "/blog/{id}",
    params(("id" = i32, Path, description = "Unique ID")),
    responses(
        (status = 200, body = BlogPost, description = "Blog post by ID"),
        (status = 404, description = "Post not found"),
    )
)]

#[get("/blog/{id}")]
async fn read_blogpost(data: web::Data<PgPool>,path: web::Path<i32>) -> Result<impl Responder, ApiError> {
    match queries::get_post(&data, path.into_inner()).await {
        Ok(post) => Ok(HttpResponse::Ok().json(post)),
        Err(e) => { 
            if let sqlx::Error::RowNotFound = e {
            Err(ApiError::NotFound("Detail Not Found".to_string()))
            } else 
            {
                Err(ApiError::from(e))
            }
        }
    }
}

#[utoipa::path(
    put,
    path = "/blog/{id}",
    params(("id" = i32, Path, description = "Unique ID")),
    request_body = BlogPost,
    responses(
        (status = 200, description = "Post updated successfully"),
        (status = 404, description = "Post not found"),
    )
)]
#[put("/blog/{id}")]
async fn update_blogpost(
    data: web::Data<PgPool>,
    path: web::Path<i32>,
    updated_post: web::Json<BlogPost>,
) -> Result<impl Responder, ApiError> {
    queries::update_post(&data, path.into_inner(), &updated_post).await.map_err(ApiError::from)?;
    Ok(HttpResponse::Ok().json(()))
}


#[utoipa::path(
    delete,
    path = "/blog/{id}",
    params(("id" = i32, Path, description = "Unique ID")),
    responses(
        (status = 200, description = "Post deleted successfully"),
        (status = 404, description = "Post not found"),
    )
)]
#[delete("/blog/{id}")]
async fn delete_blogpost(data: web::Data<PgPool>,path: web::Path<i32>) -> Result<impl Responder, ApiError> {
    match queries::delete_post(&data, path.into_inner()).await {
        Ok(_) => Ok(HttpResponse::Ok().json(())),
        Err(e) => {
            if let sqlx::Error::RowNotFound = e {
                Err(ApiError::NotFound("Detail not found".to_string()))
            } else {
                Err(ApiError::from(e))
            }
        }
    }
}


// Routes
// CRUD Routes
pub async fn index_page() -> &'static str {
    "Hello CRUD API"
}