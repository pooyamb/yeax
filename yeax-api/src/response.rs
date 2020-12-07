use std::any::TypeId;
use std::fmt;
use std::future::{ready, Ready};

use actix_web::http::StatusCode;
use actix_web::{Error, HttpRequest, HttpResponse, Responder, ResponseError};
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

#[derive(Debug)]
pub struct JsonResponse<T> {
    status: StatusCode,
    content: T,
    total: Option<usize>,
    next: Option<String>,
    prev: Option<String>,
}

impl<T> Default for JsonResponse<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            status: StatusCode::from_u16(200).unwrap(),
            content: T::default(),
            total: None,
            prev: None,
            next: None,
        }
    }
}

impl JsonResponse<()> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T> JsonResponse<T> {
    pub fn with_content(content: T) -> Self {
        Self {
            status: StatusCode::from_u16(200).unwrap(),
            content,
            total: None,
            prev: None,
            next: None,
        }
    }

    pub fn next(mut self, next: String) -> Self {
        self.next = Some(next);
        self
    }

    pub fn prev(mut self, prev: String) -> Self {
        self.prev = Some(prev);
        self
    }

    pub fn total(mut self, total: usize) -> Self {
        self.total = Some(total);
        self
    }

    pub fn content<B>(self, content: B) -> JsonResponse<B> {
        JsonResponse {
            status: self.status,
            content,
            total: self.total,
            prev: self.prev,
            next: self.next,
        }
    }
}

impl<T> fmt::Display for JsonResponse<T>
where
    T: Serialize + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("JsonResponse: ")?;
        f.write_str(&serde_json::to_string_pretty(self).map_err(|_err| fmt::Error)?)
    }
}

impl<T> Serialize for JsonResponse<T>
where
    T: Serialize + 'static,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut err = serializer.serialize_struct("JsonError", 3)?;
        err.serialize_field("status", &self.status.as_u16())?;

        if TypeId::of::<T>() == TypeId::of::<()>() {
        } else {
            err.serialize_field("content", &self.content)?;
        }

        if self.next.is_none() {
            err.skip_field("next")?;
        } else {
            err.serialize_field("next", &self.next)?;
        }

        if self.prev.is_none() {
            err.skip_field("prev")?;
        } else {
            err.serialize_field("prev", &self.prev)?;
        }

        if self.total.is_none() {
            err.skip_field("total")?;
        } else {
            err.serialize_field("total", &self.total)?;
        }

        err.end()
    }
}

impl<T> Responder for JsonResponse<T>
where
    T: 'static,
    T: Serialize,
{
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Self::Error>>;

    fn respond_to(self, _: &HttpRequest) -> Self::Future {
        ready(Ok(HttpResponse::build(self.status).json(self)))
    }
}

/////////////////////////////////////////////////////////////////////////////////////////

#[derive(Default, Debug)]
pub struct JsonError<T = ()> {
    pub status: StatusCode,
    pub code: &'static str,
    pub hint: Option<String>,
    pub content: T,
}

impl JsonError<()> {
    pub fn new(status: u16, code: &'static str) -> Self {
        Self {
            status: StatusCode::from_u16(status).unwrap(),
            code,
            hint: None,
            content: (),
        }
    }
}

impl<T> JsonError<T> {
    pub fn with_content(status: u16, code: &'static str, content: T) -> Self {
        Self {
            status: StatusCode::from_u16(status).unwrap(),
            code,
            hint: None,
            content,
        }
    }

    pub fn hint(mut self, hint: String) -> Self {
        self.hint = Some(hint);
        self
    }

    pub fn content<B>(self, content: B) -> JsonError<B> {
        JsonError {
            status: self.status,
            code: self.code,
            hint: self.hint,
            content,
        }
    }
}

impl<T> fmt::Display for JsonError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("JsonError: ")?;
        f.write_str(self.code)
    }
}

impl<T> Serialize for JsonError<T>
where
    T: Serialize + 'static,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut err = serializer.serialize_struct("JsonError", 3)?;
        err.serialize_field("status", &self.status.as_u16())?;
        err.serialize_field("code", &self.code)?;

        if self.hint.is_none() {
            err.skip_field("hint")?;
        } else {
            err.serialize_field("hint", &self.hint)?;
        }

        if TypeId::of::<T>() == TypeId::of::<()>() {
        } else {
            err.serialize_field("content", &self.content)?;
        }

        err.end()
    }
}

impl<T> ResponseError for JsonError<T>
where
    T: 'static,
    T: fmt::Debug,
    T: Serialize,
{
    fn status_code(&self) -> StatusCode {
        self.status
    }

    fn error_response(&self) -> HttpResponse {
        let mut resp = HttpResponse::build(self.status);
        resp.json(self)
    }
}

impl<T> Responder for JsonError<T>
where
    T: 'static,
    T: fmt::Display + fmt::Debug,
    T: Serialize,
{
    type Error = Self;
    type Future = Ready<Result<HttpResponse, Self>>;

    fn respond_to(self, _: &HttpRequest) -> Self::Future {
        ready(Err(self))
    }
}

#[cfg(test)]
mod test {
    use super::{JsonError, JsonResponse};

    #[actix_web::get("")]
    async fn test_error() -> Result<String, JsonError> {
        Ok("S".to_string())
    }

    #[actix_web::get("/2")]
    async fn test_response() -> JsonResponse<i32> {
        JsonResponse::with_content(10)
    }
}
