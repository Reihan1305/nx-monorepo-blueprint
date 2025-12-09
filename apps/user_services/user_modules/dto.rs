use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserDto {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub phone_number: String,
}
