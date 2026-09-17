use actix_web::{HttpResponse, Responder, get};

#[get("/ping2")]
async fn ping2() -> impl Responder {
    HttpResponse::Ok().body("pong2")
}
