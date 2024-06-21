mod config;
mod handler;
use axum::{
    routing::{get, patch, post},
    Router,
};
use std::{ops::Deref, sync::Arc};

pub use config::*;
pub use handler::*;

#[derive(Debug, Clone)]
struct AppState {
    inner: Arc<AppStateInner>,
}

#[derive(Debug)]
#[allow(dead_code)]
struct AppStateInner {
    config: AppConfig,
}

impl Deref for AppState {
    type Target = AppStateInner;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AppState {
    pub fn new(config: AppConfig) -> Self {
        Self {
            inner: Arc::new(AppStateInner { config }),
        }
    }
}
pub fn get_router(config: AppConfig) -> Router {
    let state = AppState::new(config);
    let api = Router::new()
        .route("/signin", post(signin_handler))
        .route("/signup", post(signup_handler)) // 创建一个名为api的路由器，并添加两个POST请求处理程序。
        .route("/chat", patch(chat_handler));

    Router::new() // 创建一个名为api的路由器，并添加一个GET请求处理程序。
        .route("/", get(index_handler))
        .nest("/api", api)
        .with_state(state)
}
