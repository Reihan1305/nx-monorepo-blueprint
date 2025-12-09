use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserDto {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub phone_number: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct CreateUserResponse {
    pub id: uuid::Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
}
