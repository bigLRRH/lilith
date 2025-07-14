mod analysis;
mod config;
mod data;
mod db;
mod engine;
mod error;
mod routes;
mod strategy;
mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 读取配置
    let settings = config::Settings::new()?;

    // 创建数据库连接池
    let db_pool = db::create_pool(&settings.database_url).await?;

    // 初始化 DbDataProvider
    let data_provider = data::db_data_provider::DbDataProvider::new(db_pool);

    // // 跑 engine pipeline
    // let stock_reports = engine::executor::run_pipeline(&data_provider).await;
    // for report in stock_reports {
    //     println!("{:?}", report);
    // }

    Ok(())
}
