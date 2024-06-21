mod chat;
mod user;
pub use chat::*;
pub use user::*;

pub async fn index_handler() -> &'static str {
    "Hello, axum!"
}
