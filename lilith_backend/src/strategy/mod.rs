pub mod ranker;
pub mod risk_filter;
pub mod scorer;
pub mod screener;

use crate::types::stock::Stock;

pub fn apply_strategy(mut stocks: Vec<Stock>) -> Vec<Stock> {
    stocks
        .retain(|stock| screener::passes_basic_filters(stock) && !risk_filter::is_high_risk(stock));

    ranker::rank_stocks(&mut stocks);

    stocks
}
