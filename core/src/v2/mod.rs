//! Core types and traits associated with the
//! [OpenAPI v2 specification](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md).

#[doc(inline)]
pub use paperclip_macros::*;

pub mod models;
pub mod schema;

pub use self::schema::Schema;
