mod config;
mod db;
mod error;
mod routes;

use axum::Router;
use config::Settings;
use db::create_pool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 加载配置
    let settings = Settings::new()?;

    // 创建数据库连接池
    let pool = create_pool(&settings.database_url).await?;

    // 构建路由
    let app = Router::new().with_state(pool);

    let listener = tokio::net::TcpListener::bind(settings.server_addr)
        .await
        .unwrap();

    tracing::info!("Server listening on {}", settings.server_addr);

    // 启动服务器
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
