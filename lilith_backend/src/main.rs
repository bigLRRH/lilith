mod analysis;
mod config;
mod data;
mod db;
mod engine;
mod error;
mod routes;
mod strategy;
mod types;

fn main() {
    let data_provider = data::data_provider::MockDataProvider::new();
    let stock_reports = engine::executor::run_pipeline(&data_provider);
    for report in stock_reports {
        println!("{:?}", report);
    }
}
