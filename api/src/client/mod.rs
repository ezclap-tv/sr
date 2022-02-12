use std::sync::Arc;

pub mod youtube;

pub type Client<T> = Arc<T>;
pub use youtube::YouTube;
