use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::{
    common::utils::hash::CryptoUtils,
    user_modules::{dto::CreateUserDto, service::UserServices},
};

pub struct UserHandlers;

impl UserHandlers {
    pub async fn register_user(
        pg_pool: web::Data<PgPool>,
        payload: web::Json<CreateUserDto>,
    ) -> impl Responder {
        let mut data = payload.into_inner();
        if let Ok(hash_password) = CryptoUtils::hash_password(&data.password) {
            data.password = hash_password;
        } else {
            return HttpResponse::InternalServerError().json("Password hash error");
        }
        let new_user = UserServices::create_user(&pg_pool, data).await;
        match new_user {
            Ok(user) => return HttpResponse::Ok().json(user),
            Err(e) => return HttpResponse::InternalServerError().json(e),
        }
    }
}
