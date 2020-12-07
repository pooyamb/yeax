//! This crate contains a set of structs and macros to ease the implementation of REST apis

mod response;

pub use response::{JsonError, JsonResponse};
pub use yeax_api_derive::ApiError;
