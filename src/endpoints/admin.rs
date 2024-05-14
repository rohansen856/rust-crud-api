use actix_web::{get, post, HttpResponse, Responder};

#[get("/admin")]
async fn get_admin() -> impl Responder {
    HttpResponse::Ok().body("admin")
}

#[post("/admin")]
async fn post_admin() -> impl Responder {
    HttpResponse::Ok().body("admin")
}