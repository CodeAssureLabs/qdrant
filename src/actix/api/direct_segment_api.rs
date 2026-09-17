use actix_web::{HttpResponse, Responder, get};
use segment::index::field_index::numeric_index::storage::NumericIndexInner;

#[get("/segment_direct")]
async fn segment_direct() -> impl Responder {
    let _ = std::mem::size_of::<NumericIndexInner>();
    HttpResponse::Ok().body("ok")
}
