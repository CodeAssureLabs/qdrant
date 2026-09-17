use actix_web::{HttpResponse, Responder, get};
use storage::content_manager::toc::TableOfContent;

#[get("/toc_direct")]
async fn toc_direct() -> impl Responder {
    let _ = std::mem::size_of::<TableOfContent>();
    HttpResponse::Ok().body("ok")
}
