use actix_http::StatusCode;
use actix_web::{error::ResponseError, HttpResponse};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Error {
  #[serde(skip)]
  code: StatusCode,
  message: String,
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.message)
  }
}

impl std::error::Error for Error {}

impl ResponseError for Error {
  fn status_code(&self) -> StatusCode {
    self.code
  }

  fn error_response(&self) -> HttpResponse {
    HttpResponse::build(self.status_code()).json(self)
  }
}

impl<T: IntoMsgAndCode> From<T> for Error {
  fn from(v: T) -> Self {
    let (code, message) = v.into_msg_and_code();
    Error { code, message }
  }
}

pub trait IntoMsgAndCode {
  fn into_msg_and_code(self) -> (StatusCode, String);
}

impl IntoMsgAndCode for (StatusCode, String) {
  fn into_msg_and_code(self) -> (StatusCode, String) {
    (self.0, self.1)
  }
}

impl IntoMsgAndCode for (StatusCode, &str) {
  fn into_msg_and_code(self) -> (StatusCode, String) {
    (self.0, self.1.into())
  }
}

impl IntoMsgAndCode for String {
  fn into_msg_and_code(self) -> (StatusCode, String) {
    (StatusCode::BAD_REQUEST, self)
  }
}

impl<'a> IntoMsgAndCode for &'a str {
  fn into_msg_and_code(self) -> (StatusCode, String) {
    (StatusCode::BAD_REQUEST, self.into())
  }
}

impl IntoMsgAndCode for StatusCode {
  fn into_msg_and_code(self) -> (StatusCode, String) {
    (
      self,
      self.canonical_reason().map(|v| v.to_string()).unwrap_or_else(|| {
        StatusCode::INTERNAL_SERVER_ERROR
          .canonical_reason()
          .unwrap()
          .to_string()
      }),
    )
  }
}

pub trait FailWith<T> {
  fn with(self, info: impl IntoMsgAndCode) -> std::result::Result<T, Error>;
  fn internal(self) -> std::result::Result<T, Error>;
}

impl<T, E: std::fmt::Debug> FailWith<T> for std::result::Result<T, E> {
  fn with(self, info: impl IntoMsgAndCode) -> std::result::Result<T, Error> {
    match self {
      Ok(v) => Ok(v),
      Err(_) => Err(info.into()),
    }
  }
  fn internal(self) -> std::result::Result<T, Error> {
    self.map_err(|e| {
      log::error!("Discarded internal error: {:?}", e);
      Error {
        message: "Internal Server Error".into(),
        code: StatusCode::INTERNAL_SERVER_ERROR,
      }
    })
  }
}

impl<T> FailWith<T> for std::option::Option<T> {
  fn with(self, info: impl IntoMsgAndCode) -> std::result::Result<T, Error> {
    match self {
      Some(v) => Ok(v),
      None => Err(info.into()),
    }
  }
  fn internal(self) -> std::result::Result<T, Error> {
    self.ok_or_else(|| Error {
      message: "Internal Server Error".into(),
      code: StatusCode::INTERNAL_SERVER_ERROR,
    })
  }
}
