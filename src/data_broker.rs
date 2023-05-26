pub enum DownloadError
{
    Io(String),
    Decode(String),
    Http(u16, String),
}
pub trait Downloader
{
    fn download(&self, url: &str) -> Result<String, DownloadError>;
}

pub struct DownloaderLive;
impl Downloader for DownloaderLive
{
    fn download(&self, url: &str) -> Result<String, DownloadError> {
        match ureq::get(url).call()
        {
            Ok(response) => {
                match response.into_string()
                {
                    Ok(converted_response) => Ok(converted_response),
                    Result::Err(inner_fail) => Err(DownloadError::Decode(inner_fail.to_string())),
                }
            },
            Err(ureq::Error::Status(code, response)) => {
                if let Ok(msg) = response.into_string() {
                    Err(DownloadError::Http(code, msg))
                } else {
                    Err(DownloadError::Http(code, String::from("failed to decode response")))
                }
            }
            Err(_) => {
                Err(DownloadError::Io(String::from("Generic")))
            }
        }
    }
}

use rust_decimal::Decimal;
use serde::Deserialize;
use fast_xml::de::{from_str, DeError};
use rust_decimal;

#[derive(Debug, Deserialize, PartialEq)]
pub struct FlexStatementResponse
{
    #[serde(rename = "Status", default)]
    pub status: String,
    #[serde(rename = "ReferenceCode", default)]
    pub reference_code: String,
    #[serde(rename = "Url", default)]
    pub url: String
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct FlexStatements
{
    count: i32,
    #[serde(rename = "FlexStatement", default)]
    flex_statement: Vec<FlexStatement>
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct AccountInformation
{
    #[serde(rename = "accountId", default)]
    account_id: String,
    currency: String,
    name: String,
    #[serde(rename = "primaryEmail", default)]
    primary_email: String
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Trade
{
    #[serde(rename = "accountId", default)]
    account_id: String,
    currency: String,
    #[serde(rename = "fxRateToBase", default)]
    fx_rate_to_base: Decimal,
    #[serde(rename = "assetCategory", default)]
    asset_category: String,
    symbol: String,
    description: String,
    conid: String,
    #[serde(rename = "underlyingConid", default)]
    underlying_conid: String,
    #[serde(rename = "underlyingSymbol", default)]
    underlying_symbol: String,
    multiplier: Decimal,
    // extract into enum, see https://stackoverflow.com/questions/39070244/can-i-convert-a-string-to-enum-without-macros-in-rust
    #[serde(rename = "putCall", default)]
    put_call: String,
    // extract into date type
    #[serde(rename = "tradeDate", default)]
    trade_date: String,
    quantity: Decimal,
    proceeds: Decimal,
    taxes: Decimal,
    #[serde(rename = "ibCommission", default)]
    ib_commission: Decimal,
    #[serde(rename = "netCash", default)]
    net_cash: Decimal,
    // make this enum
    #[serde(rename = "openCloseIndicator", default)]
    open_close_indicator: String,
    // make this an enum as well
    notes: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct CashTransaction
{
    #[serde(rename = "accountId", default)]
    pub account_id: String,
    pub currency: String,
    #[serde(rename = "fxRateToBase", default)]
    pub fx_rate_to_base: Decimal,
    #[serde(rename = "assetCategory", default)]
    pub asset_category: String,
    pub symbol: String,
    pub description: String,
    pub conid: String,
    pub multiplier: Decimal,
    // extract into enum
    #[serde(rename = "tradeDate", default)]
    pub trade_date: String,
    #[serde(rename = "dateTime", default)]
    pub date_time: String,
    pub amount: Decimal,
    #[serde(rename = "type", default)]
    pub type_enum: String,
}


#[derive(Debug, Deserialize, PartialEq)]
pub struct OpenPosition
{
    #[serde(rename = "accountId", default)]
    account_id: String,
    currency: String,
    #[serde(rename = "fxRateToBase", default)]
    fx_rate_to_base: Decimal,
    #[serde(rename = "assetCategory", default)]
    asset_category: String,
    symbol: String,
    description: String,
    position: Decimal,
    #[serde(rename = "markPrice", default)]
    mark_price: Decimal,
    #[serde(rename = "positionValueInBase", default)]
    position_value_in_base: Decimal,
    multiplier: Decimal,
    #[serde(rename = "costBasisPrice", default)]
    cost_basis_price: Decimal,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct OpenPositions
{
    #[serde(rename = "OpenPosition", default)]
    open_position: Vec<OpenPosition>
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct CashTransactions
{
    #[serde(rename = "CashTransaction", default)]
    cash_transaction: Vec<CashTransaction>
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Trades
{
    #[serde(rename = "Trade", default)]
    trade: Vec<Trade>
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct FlexStatement
{
    #[serde(rename = "accountId", default)]
    account_id: String,
    #[serde(rename = "fromDate", default)]
    from_date: String,
    #[serde(rename = "toDate", default)]
    to_date: String,
    #[serde(rename = "period", default)]
    period: String,
    #[serde(rename = "AccountInformation", default)]
    account_information: Option<AccountInformation>,
    #[serde(rename = "OpenPositions", default)]
    open_positions: Option<OpenPositions>,
    #[serde(rename = "Trades", default)]
    trades: Option<Trades>,
    #[serde(rename = "CashTransactions", default)]
    cash_transactions: Option<CashTransactions>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct FlexQueryResponse
{
    #[serde(rename = "queryName", default)]
    query_name: String,
    #[serde(rename = "FlexStatements", default)]
    flex_statements: Option<FlexStatements>
}

impl FlexQueryResponse {
    fn open_positions(&self) -> Option<std::slice::Iter<'_, OpenPosition>>
    {
        Some(self.flex_statements.as_ref()?.flex_statement.get(0).as_ref()?.open_positions.as_ref()?.open_position.iter())
    }
}

impl FlexQueryResponse {
    fn trades(&self) -> Option<std::slice::Iter<'_, Trade>>
    {
        Some(self.flex_statements.as_ref()?.flex_statement.get(0).as_ref()?.trades.as_ref()?.trade.iter())
    }
}

impl FlexQueryResponse {
    pub fn cash_transactions(&self) -> Option<std::slice::Iter<'_, CashTransaction>>
    {
        Some(self.flex_statements.as_ref()?.flex_statement.get(0).as_ref()?.cash_transactions.as_ref()?.cash_transaction.iter())
    }
}

pub fn read_in_prepare_statement(statement_response: &str) -> Result<FlexStatementResponse, DeError>
{
    let response: FlexStatementResponse = from_str(statement_response)?;
    Ok(response)
}

pub fn read_in_statement_response(statement_response: &str) -> Result<FlexQueryResponse, DeError> {
    let response: FlexQueryResponse = from_str(statement_response)?;
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prepare_statement() {
        let path = concat!(std::env!("CARGO_MANIFEST_DIR"), "/tests/resources/", "prepare_statement_success.xml");
        let contents = std::fs::read_to_string(path).expect("Should have been able to read the file");
        let response = read_in_prepare_statement(contents.as_str()).unwrap();
        println!("reference code is: {}", response.reference_code);
        println!("url is: {}", response.url);
        assert!(response.status == "Success");
    }

    #[test]
    fn test_return_statement() {
        let path = concat!(std::env!("CARGO_MANIFEST_DIR"), "/tests/resources/", "retrieve_full_query_result.xml");
        let contents = std::fs::read_to_string(path).expect("Should have been able to read the file");
        let response: FlexQueryResponse = read_in_statement_response(contents.as_str()).unwrap();
        assert!(response.query_name == "parser");
        let flex_statements = response.flex_statements.unwrap();
        assert!(flex_statements.count == 1);
        let flex_statements_vec = flex_statements.flex_statement;
        let flex_statement_1 = flex_statements_vec.get(0).unwrap();
        assert!(flex_statement_1.account_id == "U5241807");
        let first_position = flex_statement_1.open_positions.as_ref().unwrap().open_position.get(0).unwrap();
        assert!(first_position.currency == "EUR");
        assert!(first_position.symbol == "MC");
        assert!(first_position.asset_category == "STK");
        println!("position amount: {}", first_position.position);
        assert!(first_position.position.to_string() == "30");
    }

    #[test]
    fn dump_open_positions(){
        let path = concat!(std::env!("CARGO_MANIFEST_DIR"), "/tests/resources/", "retrieve_full_query_result.xml");
        let contents = std::fs::read_to_string(path).expect("Should have been able to read the file");
        let response: FlexQueryResponse = read_in_statement_response(contents.as_str()).unwrap();
        for open_position in response.open_positions().unwrap()
        {
            println!("open position: {}", open_position.description)
        }
    }

    #[test]
    fn dump_trades(){
        let path = concat!(std::env!("CARGO_MANIFEST_DIR"), "/tests/resources/", "retrieve_full_query_result.xml");
        let contents = std::fs::read_to_string(path).expect("Should have been able to read the file");
        let response: FlexQueryResponse = read_in_statement_response(contents.as_str()).unwrap();
        for trades in response.trades().unwrap()
        {
            println!("traded: {} of {}", trades.quantity, trades.description)
        }
    }

    #[test]
    fn dump_cash_transactions(){
        let path = concat!(std::env!("CARGO_MANIFEST_DIR"), "/tests/resources/", "retrieve_full_query_result.xml");
        let contents = std::fs::read_to_string(path).expect("Should have been able to read the file");
        let response: FlexQueryResponse = read_in_statement_response(contents.as_str()).unwrap();
        for cash_transaction in response.cash_transactions().unwrap()
        {
            println!("cash transaction: {} for amount {}", cash_transaction.description, cash_transaction.amount)
        }
    }
}


pub fn retrieve_flex_statement(downloader: Box<dyn Downloader>, token: &str, query: &str) -> Result<FlexQueryResponse, DownloadError>
{
    const REQUEST_VERSION :u32 = 3;
    let order_flex = format!("https://gdcdyn.interactivebrokers.com/Universal/servlet/FlexStatementService.SendRequest?t={}&q={}&v={}", token, query, REQUEST_VERSION);
    match (*downloader).download(&order_flex)
    {
        Ok(response) => 
        {
            match from_str::<FlexStatementResponse>(&response)
            {
                Ok(flex_response) => {
                    let final_download_url = format!("{}?q={}&t={}&v={}", flex_response.url, query, token, REQUEST_VERSION);
                    match (*downloader).download(&final_download_url)
                    {
                        Ok(response) =>
                        {
                            match from_str::<FlexQueryResponse>(&response)
                            {
                                Ok(query_response) => Ok(query_response),
                                Err(_) => Err(DownloadError::Decode(String::from("Failed parsing flex query response"))),
                            }
                        },
                        Err(error) => Err(error),
                    }
                },
                Err(_) => Err(DownloadError::Decode(String::from("Failed parsing flex statement response"))),
            }
        },
        Err(error) => Err(error),
    }
}
