use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Error(eyre::Report);

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let json = serde_json::json!({
            "error": {
                "message": self.0.to_string(),
            },
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json)).into_response()
    }
}

impl From<eyre::Report> for Error {
    fn from(error: eyre::Report) -> Self {
        Error(error)
    }
}
