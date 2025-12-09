use actix_web::{web, HttpResponse, Responder};

use crate::user_modules::dto::CreateUserDto;

pub struct UserHandlers;

impl UserHandlers {
    pub async fn register_user(payload: web::Json<CreateUserDto>) -> impl Responder {
        let data = payload.into_inner();
        return HttpResponse::Ok().json(data);
    }
}
