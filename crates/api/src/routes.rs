use actix_web::web;

use crate::views::{
    order_progs,
    tag_progs,
    serve_progs,
    search_progs,
    pages,
    progs,
    auth,
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
    .configure(pages::pages_routes)
    .configure(progs::progs_routes)
    .configure(search_progs::search_routes)
    .configure(serve_progs::serve_routes)
    .configure(tag_progs::tag_routes)
    .configure(auth::auth_routes)
    .configure(order_progs::order_routes)
    ;
}
