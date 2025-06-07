#[derive(Debug)]
pub struct StockReport {
    pub stock_id: i64,
    pub stock_exchange: String,
    pub stock_ticker: String,
    pub stock_symbol: String,
    pub stock_company_name: String,
    pub stock_short_name: Option<String>,
    pub stock_ipo_date: Option<chrono::NaiveDate>,
    pub stock_delist_date: Option<chrono::NaiveDate>,
    pub stock_status: String,
    pub stock_created_at: chrono::DateTime<chrono::Utc>,
    pub stock_updated_at: chrono::DateTime<chrono::Utc>,
    pub report_date: chrono::NaiveDate,
    pub report_type: String,
    pub report_content: String,
}
