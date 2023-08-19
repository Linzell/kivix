use actix_web::{http::StatusCode, HttpResponse, ResponseError};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Fail to get Ctx")]
    CtxFail,

    #[error("Fetching from Untis failed")]
    UntisError,

    #[error(transparent)]
    Surreal(#[from] surrealdb::Error),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
}

impl ResponseError for Error {

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let code = self.status_code();
        match code {
            StatusCode::NOT_FOUND => HttpResponse::NotFound().body(format!("404 Not Found\n{self}")),
            StatusCode::FORBIDDEN => HttpResponse::Forbidden().body(format!("403 Forbidden\n{self}")),
            StatusCode::CONFLICT => HttpResponse::Conflict().body(format!("409 Conflict\n{self}")),
            StatusCode::INTERNAL_SERVER_ERROR => {
                HttpResponse::InternalServerError().body(format!("500 Internal Server Error\n{self}"))
            }
            // These 2⭥ are seperate if we ever wanna change the default/add a new code
            _ => HttpResponse::InternalServerError().body(format!("500 Internal Server Error\n{self}")),
        }
    }
}