use sqlx::PgPool;
use crate::{data::data_provider::DataProvider, types::stock::Stock};
use tracing::error; // Add tracing for structured logging

pub struct DbDataProvider {
    db_pool: PgPool,
}

impl DbDataProvider {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }
}

// #[async_trait::async_trait]
// impl DataProvider for DbDataProvider {
//     async fn get_all_stocks(&self) -> Vec<Stock> {
//         match sqlx::query_as!(
//             Stock,
//             r#"
//             SELECT id, exchange, ticker, symbol, company_name, short_name,
//                    ipo_date, delist_date, status, created_at, updated_at
//             FROM stocks
//             "#
//         )
//         .fetch_all(&self.db_pool)
//         .await
//         {
//             Ok(stocks) => stocks,
//             Err(e) => {
//                 // Log detailed error
//                 error!("Failed to fetch stocks: {}", e);
//                 Vec::new() // Fallback to empty vector
//             }
//         }
//     }
// }