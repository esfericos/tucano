use std::borrow::Cow;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use tracing::error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Error {
    public: Option<(StatusCode, Cow<'static, str>)>,
    inner: eyre::Report,
}

impl Error {
    /// Constructs a new error with a public message and a status code.
    #[track_caller]
    pub fn public(status: StatusCode, msg: impl Into<Cow<'static, str>>) -> Self {
        let msg = msg.into();
        Self::public_with(eyre::Report::msg(msg.clone()), status, msg)
    }

    /// Constructs a new error with a inner error, a public message, and a
    /// status code.
    pub fn public_with(
        inner: impl Into<eyre::Report>,
        status: StatusCode,
        msg: impl Into<Cow<'static, str>>,
    ) -> Self {
        let msg = msg.into();
        Self {
            inner: inner.into(),
            public: Some((status, msg)),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, message) = self
            .public
            .unwrap_or((StatusCode::INTERNAL_SERVER_ERROR, "internal error".into()));

        let json = serde_json::json!({
            "error": {
                "message": message,
            },
        });

        let error = self.inner;
        if StatusCode::INTERNAL_SERVER_ERROR <= status {
            error!(?error, %status, "server error http response");
        }
        (status, Json(json)).into_response()
    }
}

impl From<eyre::Report> for Error {
    fn from(inner: eyre::Report) -> Self {
        let public = None;
        Error { public, inner }
    }
}

pub trait ResultExt<T, E> {
    fn http_error(self, status: StatusCode, msg: impl Into<Cow<'static, str>>) -> Result<T, Error>;
}

impl<T, E> ResultExt<T, E> for Result<T, E>
where
    E: Into<eyre::Report>,
{
    #[track_caller]
    fn http_error(self, status: StatusCode, msg: impl Into<Cow<'static, str>>) -> Result<T, Error> {
        match self {
            Ok(ok) => Ok(ok),
            Err(error) => Err(Error {
                public: Some((status, msg.into())),
                inner: error.into(),
            }),
        }
    }
}

pub trait OptionExt<T> {
    fn or_http_error(
        self,
        status: StatusCode,
        msg: impl Into<Cow<'static, str>>,
    ) -> Result<T, Error>;
}

impl<T> OptionExt<T> for Option<T> {
    #[track_caller]
    fn or_http_error(
        self,
        status: StatusCode,
        msg: impl Into<Cow<'static, str>>,
    ) -> Result<T, Error> {
        match self {
            Some(some) => Ok(some),
            None => Err(Error::public(status, msg)),
        }
    }
}
