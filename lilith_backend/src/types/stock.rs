use sqlx::FromRow;

#[derive(Debug, FromRow, Clone)]
pub struct Stock {
    pub id: i64,
    pub exchange: String,
    pub ticker: String,
    pub symbol: String,
    pub company_name: String,
    pub short_name: Option<String>,
    pub ipo_date: Option<chrono::NaiveDate>,
    pub delist_date: Option<chrono::NaiveDate>,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}