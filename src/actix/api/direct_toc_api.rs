use actix_web::{HttpResponse, Responder, get, web};
use collection::operations::verification::new_unchecked_verification_pass;
use storage::dispatcher::Dispatcher;

use crate::actix::auth::ActixAuth;
use crate::common::toc_summary::do_toc_summary;

#[get("/toc_direct")]
async fn toc_direct(
    dispatcher: web::Data<Dispatcher>,
    ActixAuth(auth): ActixAuth,
) -> impl Responder {
    // No request to verify
    let pass = new_unchecked_verification_pass();

    let summary = do_toc_summary(dispatcher.toc(&auth, &pass), &auth).await;

    HttpResponse::Ok().body(format!("ok: {} collections", summary.collections))
}

pub fn config_direct_toc_api(cfg: &mut web::ServiceConfig) {
    cfg.service(toc_direct);
}
