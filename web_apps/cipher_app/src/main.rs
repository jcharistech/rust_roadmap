use actix_web::{middleware::Logger,web,App,HttpRequest,HttpResponse,Responder,Result,HttpServer,post};
use env_logger;
use tera::Tera;
use lazy_static::lazy_static;
use serde::{Deserialize};
use rand::{thread_rng,Rng};
use rand_distr::Alphanumeric;

// Templating
lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let source = "templates/**/*";
        let tera = Tera::new(source).unwrap();
        tera
    };
}

fn generate_pass(text: &str,password_len: i32) -> String {
    let mut rng = rand::thread_rng();
    let rand_str: String = (0..password_len).map(|_| rng.sample(Alphanumeric) as char).collect();
    let result = format!("{}{}",text.to_lowercase(),rand_str);
    return result
}


// Endpoint
async fn home_page(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("rust world");
    format!("Hello Rustaceans {}",&name)
}

// Endpoint with HTML Templating
async fn about_page() -> impl Responder {
    let mut context = tera::Context::new();
    context.insert("name","About Page");
    match TEMPLATES.render("about.html", &context) {
        Ok(page) => HttpResponse::Ok().body(page),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error rendering template: {}",e)),
    }
}


async fn index_page() -> impl Responder {
    let mut context = tera::Context::new();
    context.insert("name","Home Page");
    context.insert("raw_text","");
    context.insert("result","");
    match TEMPLATES.render("index.html", &context) {
        Ok(page) => HttpResponse::Ok().body(page),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error rendering template: {}",e)),
    }

}

// Struct to collect data
#[derive(Deserialize)]
struct FormData {
    raw_text: String,
    num: i32,
}

// Using Macro
#[post("/generate")]
async fn generate_page(form: web::Form<FormData>) -> impl Responder {
    let form_data = form.into_inner();
    let raw_text = form_data.raw_text;
    let num = form_data.num;
    let result = generate_pass(&raw_text,num);
    let mut context = tera::Context::new();
    context.insert("name","Home Page");
    context.insert("raw_text",&raw_text);
    context.insert("result",&result);
    match TEMPLATES.render("index.html", &context) {
        Ok(page) => HttpResponse::Ok().body(page),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error rendering template: {}",e)),
    }

}




#[actix_web::main]
async fn main() -> Result<(),std::io::Error>{
    if std::env::var_os("RUST_LOG").is_none(){
        std::env::set_var("RUST_LOG","actix-web=info");
    }
    dotenv::dotenv().ok();
    env_logger::init();

    HttpServer::new(|| {
        App::new()
        .wrap(Logger::default())
        .route("/home", web::get().to(home_page))
        .route("/about",web::get().to(about_page))
        .route("/",web::get().to(index_page))
        .service(generate_page)
        
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await

}
