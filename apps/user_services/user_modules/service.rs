use sqlx::PgPool;

use crate::{
    common::error::{AppError, AppResult},
    user_modules::{
        dto::{CreateUserDto, CreateUserResponse},
        repo::UserRepo,
    },
};

pub struct UserServices;

impl UserServices {
    pub async fn create_user(
        pg_pool: &PgPool,
        user: CreateUserDto,
    ) -> AppResult<CreateUserResponse> {
        let db_response: Result<CreateUserResponse, sqlx::Error> =
            UserRepo::create_user(pg_pool, user).await;
        match db_response {
            Ok(response) => Ok(response),
            Err(e) => {
                return Err(map_db_error(e));
            }
        }
    }
}

use sqlx::Error;

pub fn map_db_error(err: Error) -> AppError {
    print!("err: {}", err);
    if let Error::Database(db_err) = &err {
        let code = db_err.code().unwrap_or_default();
        let constraint = db_err.constraint().unwrap_or_default();
        print!("code: {}, constraint: {}", code, constraint);
        if code == "23505" {
            let parts: Vec<&str> = constraint.split('_').collect();
            let field: String;
            if parts.len() < 3 {
                field = constraint.to_string();
            } else {
                let field_parts = &parts[1..parts.len() - 1];

                field = field_parts.join(" ");
            }

            return AppError::BadRequest(format!("{} already exists", field).into());
        }
    }

    AppError::InternalError(err.to_string())
}
