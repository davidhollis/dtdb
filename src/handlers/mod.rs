use axum::response::{IntoResponse, Html};
use hyper::StatusCode;

pub mod seasons;
pub mod toaster;

type HandlerResult = Result<Html<String>, HandlerError>;

struct HandlerError(anyhow::Error);

impl IntoResponse for HandlerError {
    // TODO: not all errors ought to be 500s. Consider doing a bit of
    //       introspection or downcasting to find examples that should
    //       be e.g., 404s or 401s
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("An unexpected error occurred: {}", self.0),
        ).into_response()
    }
}

impl<E> From<E> for HandlerError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}