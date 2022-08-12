

/*
#[cfg(test)]
mod document_tests{
    use std::env;

    use super::*;
    use crate::*;
    #[test]
    fn test_get_bank_statements() {
        dotenv::dotenv().unwrap();
        let api_key = env::var("LEMON_MARKET_TRADING_API_KEY").unwrap();
        let client = client::Client::paper_client(&api_key);
        let resp = client
            .get_bank_statements(None, None, None, None, None)
            .unwrap();
        dbg!(&resp);
    }
}
*/