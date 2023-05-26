fn main() {
    let arguments = std::env::args();
    let arguments = arguments::parse(arguments).unwrap();

    let token = arguments.get::<String>("token").expect("Failed to read token"); 
    let query = dbg!(arguments.get::<String>("query")).expect("Failed to read query");
    println!("passed token: {token}");
    println!("passed query: {query}");

    let context = midas_scribble::Context{
        strategy: midas_scribble::data_broker::DownloaderLive,
    };

    match midas_scribble::analyze(&context, &token, &query)
    {
        Ok(success) => println!("{}", success),
        Err(fail) => match fail {
            midas_scribble::data_broker::DownloadError::Io(msg) => println!("io error during download: {}", msg),
            midas_scribble::data_broker::DownloadError::Decode(msg) => println!("failed to parse response: {}", msg),
            midas_scribble::data_broker::DownloadError::Http(code, _) => println!("Failed to download with code {}", code),
        },
    }
}
