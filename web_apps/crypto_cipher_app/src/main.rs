use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Responder,post};
use serde::{Serialize, Deserialize};
use std;
use tera::Tera;
use lazy_static::lazy_static;

// Templating
lazy_static! {
    pub static ref TEMPLATES: Tera={
        let source = "templates/**/*";
        let tera = Tera::new(source).unwrap();
        tera
    };
}


// Symmetric substitution cipher: encrypt and decrypt
fn rot13_encrypt(text:&str) -> String{
    let encrypted_text = text.to_uppercase();
    encrypted_text
    .chars()
    .map(|c| match c {
        'A'..='M' => ((c as u8) + 13) as char,
        'N'..='Z' => ((c as u8) - 13) as char,
        _ => c,

    }).collect()
}

// Caesar Shift (rotX, rot16)
fn caesar_encrypt(text: &str, shift: u8) -> String{
    let mut encrypted_text = String::new();
    for char in text.chars(){
        if char.is_ascii_alphabetic(){
            let base = if char.is_ascii_uppercase() {'A'} else {'a'};
            let shifted = ((char as u8 - base as u8 + shift ) % 26) + base as u8;
            encrypted_text.push(shifted as char);
        } else {
            encrypted_text.push(char)
        }
    }
    encrypted_text

}

fn reverse_encrypt(text: &str)  ->String {
    text.chars().rev().collect()
}


fn gematric_shift_encrypt(text: &str) -> String{
    // Alpha to Numbers and join all numbers
    let mut encrypted_text = String::new();
    for char in text.chars(){
        if char.is_ascii_alphabetic(){
            let base = if char.is_ascii_uppercase() {'A' as u32 } else {'a' as u32};
            let number = (char as u32 -base + 1) as u8;
            encrypted_text.push_str(&number.to_string());
            encrypted_text.push(' ');
        }
    }
    encrypted_text.trim().to_string()
}

fn simple_gematria(text: &str) -> u32 {
    let mut gematria = 0;
    for char in text.to_lowercase().chars(){
        if char.is_alphabetic(){
            let value = (char as u32 - 'a' as u32 + 1) as u32;
            gematria += value;
        }
    }
    gematria
}


fn encrypted_text(text: &str,cipher_type:&str) -> String{
    match cipher_type {
        "rot13_encrypt" => rot13_encrypt(text),
        "reverseText" => reverse_encrypt(text),
        "caesar_encrypt" => caesar_encrypt(text, 16),
        "gematria_encrypt" => gematric_shift_encrypt(text),
        _ => rot13_encrypt(text)
    }
}

// Version 1
async fn index_page() -> &'static str {
    "Hello Rust Cipher App"
}

// With Templating 
async fn cipher_home_page() -> impl Responder{
    let mut context = tera::Context::new();
    context.insert("page_title", "Home Page");
    context.insert("message","");
    context.insert("results","");
    context.insert("text_len","");
    match TEMPLATES.render("index.html", &context) {
        Ok(page) => HttpResponse::Ok().body(page),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error rendering template: {}",e)),
    }
}

// Struct to collect data via forms
#[derive(Serialize, Deserialize, Debug)]
struct CipherFormData{
    message: String,
    cipher_type: String,
}


// Via Decorators
#[post("/cipherize")]
async fn cipher_post_page(form: web::Form<CipherFormData>) -> impl Responder{
    let form_data = form.into_inner();
    let message: String = form_data.message;
    let cipher_text: String = form_data.cipher_type;
    let text_len: usize = message.len();
    let results = encrypted_text(&message,&cipher_text);
    let mut context = tera::Context::new();
    context.insert("page_title", "Home Page");
    context.insert("message",&message);
    context.insert("results",&results);
    context.insert("text_len",&text_len);
    match TEMPLATES.render("index.html", &context) {
        Ok(page) => HttpResponse::Ok().body(page),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error rendering template: {}",e)),
    }
}






#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(||{
        App::new()
        .route("/", web::get().to(index_page))
        .route("/home", web::get().to(cipher_home_page))
        .service(cipher_post_page)
        
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
