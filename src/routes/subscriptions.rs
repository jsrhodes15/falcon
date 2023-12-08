use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}



// using serde and our FormData struct to automagically attempt deserialization of payload, returning 200 when ok and 400 if something goes wrong
pub async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}