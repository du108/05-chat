use anyhow::Result;
use notify_server::get_router;
use tokio::net::TcpListener;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer};

#[tokio::main]
async fn main() -> Result<()> {
    let console = tracing_subscriber::fmt::layer()
        .pretty() // 完成日志系统的初始化。
        .with_filter(LevelFilter::INFO); // 设置日志级别为INFO。

    tracing_subscriber::registry().with(console).init();

    let addr = "0.0.0.0:6687";

    let app = get_router(); // 获取路由。

    info!("listen on {:?}", addr); // 打印日志。
    let listener = TcpListener::bind(addr).await?; // 绑定地址和端口。

    if let Err(e) = axum::serve(listener, app.into_make_service()).await {
        panic!("{:?}", e); // 处理错误。
    }
    Ok(())
}
