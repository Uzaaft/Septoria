// TODO: Implement the code below. For now it's just commented out
use crate::client::Client;

/*
#[derive(Deserialize, Serialize, Debug)]
pub struct BankStatementResponse {
    pub id: String,
    pub account_id: String,
    #[serde(rename = "type")]
    pub bankstatementtypes: BankStatementTypes,
    pub date: String,
    pub amount: i64,
    pub isin: String,
    pub isin_title: String,
    pub created_at: String,
    pub quantity: i64,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum BankStatementTypes {
    PayIn,
    PayOut,
    OrderBuy,
    OrderSell,
    EodBalance,
    Dividend,
    TaxRefund,
}



// Porobably the easiest way  to implement Display.
// TODO: This is a bit of a hack, but it might work for now.
impl fmt::Display for BankStatementTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match to_variant_name(self) {
            Ok(name) => write!(f, "{}", name),
            _ => write!(f, ""),
        }
    }
}
impl Client{
    /// Get bank statements
    pub fn get_bank_statements(
        &self,
        statement_type: Option<BankStatementTypes>,
        from: Option<String>,
        to: Option<String>,
        sorting: Option<Sorting>,
        limit: Option<i32>,
    ) -> Result<PaginationResponse<BankStatementResponse>, Error> {
        let path = "account/bankstatements/";
        let mut query = vec![];

        if let Some(statement_type) = statement_type {
            if let Ok(query_string) =
                serde_urlencoded::to_string(&vec![("type", statement_type.to_string())])
            {
                query.push(query_string);
            }
        }
        if let Some(from) = from {
            if let Ok(query_string) = serde_urlencoded::to_string(&vec![("from", from)]) {
                query.push(query_string);
            }
        }
        if let Some(to) = to {
            if let Ok(query_string) = serde_urlencoded::to_string(&vec![("to", to)]) {
                query.push(query_string);
            }
        }
        if let Some(sorting) = sorting {
            if let Ok(query_string) =
                serde_urlencoded::to_string(&vec![("sorting", sorting.to_string())])
            {
                query.push(query_string);
            }
        }

        if let Some(limit) = limit {
            if let Ok(query_string) =
                serde_urlencoded::to_string(&vec![("limit", limit.to_string())])
            {
                query.push(query_string);
            }
        }
        let resp = self
            .get_with_query::<PaginationResponse<BankStatementResponse>, Vec<String>>(path, query);
        match resp {
            Ok(r) => Ok(r),
            // Err(e) => Err(e),
            Err(e) => Err(Error::Str(e.to_string())),
        }
    }
}

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