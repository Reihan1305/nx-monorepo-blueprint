use actix_web::web;

pub mod dto;
pub mod handler;
pub mod repo;
pub mod service;

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users").route("/", web::post().to(handler::UserHandlers::register_user)),
    );
}
