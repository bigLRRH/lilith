use crate::types::stock::Stock;

pub trait DataProvider {
    fn get_all_stocks(&self) -> Vec<Stock>;
}

pub struct MockDataProvider;
impl MockDataProvider {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl DataProvider for MockDataProvider {
    fn get_all_stocks(&self) -> Vec<Stock> {
        vec![
            Stock {
                id: 1,
                exchange: "NASDAQ".to_string(),
                ticker: "AAPL".to_string(),
                symbol: "AAPL".to_string(),
                company_name: "Apple Inc.".to_string(),
                short_name: Some("Apple".to_string()),
                ipo_date: Some(chrono::NaiveDate::from_ymd_opt(1980, 12, 12).unwrap()),
                delist_date: None,
                status: "active".to_string(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            },
            Stock {
                id: 2,
                exchange: "NASDAQ".to_string(),
                ticker: "GOOGL".to_string(),
                symbol: "GOOGL".to_string(),
                company_name: "Alphabet Inc.".to_string(),
                short_name: Some("Google".to_string()),
                ipo_date: Some(chrono::NaiveDate::from_ymd_opt(2004, 8, 19).unwrap()),
                delist_date: None,
                status: "active".to_string(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            },
            Stock {
                id: 3,
                exchange: "NYSE".to_string(),
                ticker: "TSLA".to_string(),
                symbol: "TSLA".to_string(),
                company_name: "Tesla Inc.".to_string(),
                short_name: Some("Tesla".to_string()),
                ipo_date: Some(chrono::NaiveDate::from_ymd_opt(2010, 6, 29).unwrap()),
                delist_date: None,
                status: "active".to_string(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            },
        ]
    }
}
