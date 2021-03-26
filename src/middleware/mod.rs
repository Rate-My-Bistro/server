//! The middleware crate provides a set of interceptors
//! being executed before OR after a request:
//!
//! 1. it maps given parameters into parsable structs
//! 2. it verifies the correctness of parameters
//! 3. it injects additional entities into handler
//! 4. it wraps data into a common response

pub mod api_response;
pub mod arango_pool;
pub mod date_query;
