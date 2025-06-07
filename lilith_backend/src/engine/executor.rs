use crate::{data::data_provider::DataProvider, strategy, types::stock_report::StockReport};

pub fn run_pipeline(data_provider: &dyn DataProvider) -> Vec<StockReport> {
    let stocks = data_provider.get_all_stocks();
    let selected_stocks = strategy::apply_strategy(stocks);
    let stock_reports: Vec<StockReport> = selected_stocks
        .into_iter()
        .map(|stock| StockReport {
            stock_id: stock.id,
            stock_exchange: stock.exchange.clone(),
            stock_ticker: stock.ticker.clone(),
            stock_symbol: stock.symbol.clone(),
            stock_company_name: stock.company_name.clone(),
            stock_short_name: stock.short_name.clone(),
            stock_ipo_date: stock.ipo_date,
            stock_delist_date: stock.delist_date,
            stock_status: stock.status.clone(),
            stock_created_at: stock.created_at,
            stock_updated_at: stock.updated_at,
            report_date: chrono::Utc::now().date_naive(),
            report_type: "daily".to_string(), // Example report type
            report_content: "Sample report content".to_string(), // Placeholder content
        })
        .collect();
    stock_reports
}
