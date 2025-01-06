use std::{fmt, path};
use actix_web::{middleware::Logger, web, App, HttpServer, Responder,HttpResponse,post,get,put,delete};
use env_logger;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};
use sqlx::FromRow;
use sqlx::Error as SqlxError;
use actix_web::http::StatusCode;
use actix_web::error::ResponseError;
use utoipa::{OpenApi,ToSchema};
use utoipa_swagger_ui::SwaggerUi;


// Create DB Fxn for connection
pub async fn establish_connection() -> Result<PgPool, sqlx::Error> {
    let database_url = "postgres://postgres:password123@localhost:5432/rustdb";
    PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await
}


// Model
#[derive(Serialize,Deserialize,Debug,FromRow,ToSchema)]
pub struct BlogPost {
    id: i32,
    title: String,
    content: String,
    author: String,
}


// Input Serializer
#[derive(Serialize,Deserialize,Debug,FromRow,ToSchema)]
pub struct NewBlogPost {
    #[schema(example ="Rust Crud Web App")]
    pub title: String,

    #[schema(example ="Rust Crud Web App COntent")]
    pub content: String,

    #[schema(example ="JCharis")]
    pub author: String,
}


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



// Error Handling

#[derive(Debug, Serialize, Deserialize)]
enum ApiError {
    InternalError(String),
    DatabaseError(String),
    NotFound(String),
    ValidationError(String),
}

impl ResponseError for ApiError {
    fn error_response(&self) -> actix_web::HttpResponse {
        match self {
            ApiError::InternalError(msg) => actix_web::HttpResponse::InternalServerError().json(msg),
            ApiError::DatabaseError(msg) => actix_web::HttpResponse::InternalServerError().json(msg),
            ApiError::NotFound(msg) => actix_web::HttpResponse::NotFound().json(msg),
            ApiError::ValidationError(msg) => actix_web::HttpResponse::BadRequest().json(msg),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::ValidationError(_) => StatusCode::BAD_REQUEST,
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiError::InternalError(msg) => write!(f, "Internal Server Error: {}", msg),
            ApiError::DatabaseError(msg) => write!(f, "Database Error: {}", msg),
            ApiError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            ApiError::ValidationError(msg) => write!(f, "Validation Error: {}", msg),
        }
    }
}

impl From<SqlxError> for ApiError {
    fn from(e: SqlxError) -> Self {
        ApiError::DatabaseError(format!("Database error: {}", e))
    }
}



// API Configuration and Documentation
#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "RUST CRUD API",description="RUST Actix-web and SQLX CRUD API")
    ),
    paths(get_all_blogposts,create_blogpost,read_blogpost,update_blogpost,delete_blogpost)
)]
struct ApiDoc;




// Routes
// CRUD Routes
async fn index_page() -> &'static str {
    "Hello CRUD API"
}


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
    let post = create_post(&data, &new_post).await.expect("Failed");
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
    let posts = get_posts(&data).await.map_err(ApiError::from)?;
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
    match get_post(&data, path.into_inner()).await {
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
    update_post(&data, path.into_inner(), &updated_post).await.map_err(ApiError::from)?;
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
    match delete_post(&data, path.into_inner()).await {
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




#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    // Server
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info");
    }
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Database Configuration
    let pool  = establish_connection().await.expect("Failed To Connect");
    let openapi = ApiDoc::openapi();

    HttpServer::new(move|| {
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .wrap(Logger::default())
        .route("/",web::get().to(index_page))
        .service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()))
        .service(create_blogpost)
        .service(get_all_blogposts)
        .service(read_blogpost)
        .service(update_blogpost)
        .service(delete_blogpost)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
