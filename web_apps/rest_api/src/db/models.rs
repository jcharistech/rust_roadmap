
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

// Model
#[derive(Serialize,Deserialize,Debug,FromRow,ToSchema)]
pub struct BlogPost {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub author: String,
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