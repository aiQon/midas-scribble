use data_broker::{Downloader, DownloadError, FlexStatementResponse, read_in_prepare_statement, FlexQueryResponse, read_in_statement_response};

pub mod data_broker;

pub struct Context<S> {
    pub strategy: S,
}

impl<S> Context<S>
where
    S: Downloader,
{
    pub fn download(&self, url: &str) -> Result<String, DownloadError> {
        self.strategy.download(url)
    }
}

const REQUEST_VERSION: u32 = 3;

fn order_query<S>(context: &Context<S>, token: &str, query: &str) -> Result<FlexStatementResponse, DownloadError>
where S: Downloader,
{
    let order_url = format!("https://gdcdyn.interactivebrokers.com/Universal/servlet/FlexStatementService.SendRequest?t={}&q={}&v={}", token, query, REQUEST_VERSION);
    match context.download(&order_url)
    {
        Ok(success) =>
        {
            // println!("{}", success);
            match read_in_prepare_statement(&success)
            {
                Ok(parsed) =>
                {
                    Ok(parsed)

                },
                Err(error) => {
                    Err(DownloadError::Decode(error.to_string()))
                },
            }
        },
        Err(error) => Err(error),
    }
}

fn download_flexresponse<S>(context: &Context<S>, statement_reposone: FlexStatementResponse, token: &str) -> Result<FlexQueryResponse, DownloadError>
where S: Downloader,
{
    let download_url = format!("{}?q={}&t={}&v={}", statement_reposone.url, statement_reposone.reference_code, token, REQUEST_VERSION);
    match context.download(&download_url)
    {
        Ok(success) =>
        {
            // println!("{}", success);
            match read_in_statement_response(&success)
            {
                Ok(parsed) =>
                {
                    Ok(parsed)

                },
                Err(error) => {
                    Err(DownloadError::Decode(error.to_string()))
                },
            }
        },
        Err(error) => Err(error),
    }
}

fn parse(flex_response: FlexQueryResponse) -> String
{
    match flex_response.cash_transactions()
    {
        Some(cash_transactions) =>
        {
            for cash_transaction in cash_transactions
            {
                println!("cash transaction: {} for amount {}", cash_transaction.description, cash_transaction.amount)
            }
            String::from("value")
        },
        None => String::from("empty"),
    }
}

pub fn analyze<S>(context: &Context<S>, token: &str, query: &str) -> Result<String, DownloadError>
where S: Downloader,
{
    match order_query(context, token, query)
    {
        Ok(flex_order) => 
        {
            match download_flexresponse(context, flex_order, token)
            {
                Ok(flex_response) => Ok(parse(flex_response)),
                Err(error) => Err(error),
            }
        },
        Err(error) => Err(error),
    }
}
