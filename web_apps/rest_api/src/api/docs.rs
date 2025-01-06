use utoipa::OpenApi;


// API Configuration and Documentation
#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "RUST CRUD API",description="RUST Actix-web and SQLX CRUD API")
    ),
    paths(
        crate::api::routes::create_blogpost,
        crate::api::routes::get_all_blogposts,
        crate::api::routes::read_blogpost,
        crate::api::routes::update_blogpost,
        crate::api::routes::delete_blogpost
    )
)]
pub struct ApiDoc;