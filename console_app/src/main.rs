use tiberius::{Client, Config, AuthMethod};
use async_std::net::TcpStream;

#[async_std::main]
async fn main() -> anyhow::Result<()> {

    // Using the builder method to construct the options.
    let mut config = Config::new();

    config.host("localhost");
    config.port(1433);
    config.trust_cert(); // trusting by default

    // Using SQL Server authentication.
    config.authentication(AuthMethod::sql_server("plugin", "Y0djcnm"));

    // Taking the address from the configuration, using async-std's
    // TcpStream to connect to the server.
    let tcp = TcpStream::connect(config.get_addr()).await?;
     
    // We'll disable the Nagle algorithm. Buffering is handled
    // internally with a `Sink`.
    tcp.set_nodelay(true)?;

    // Handling TLS, login and other details related to the SQL Server.
    let mut client = Client::connect(config, tcp).await?;

    // A response to a query is a stream of data, that must be
    // polled to the end before querying again. Using streams allows
    // fetching data in an asynchronous manner, if needed.
    let mut stream = client.query("SELECT @P1", &[&-4i32]).await?;

    // As long as the `next_resultset` returns true, the stream has
    // more results and can be polled. For each result set, the stream
    // returns rows until the end of that result. In a case where
    // `next_resultset` is true, polling again will return rows from
    // the next query.
    assert!(stream.next_resultset());
     
    // In this case, we know we have only one query, returning one row
    // and one column, so calling `into_row` will consume the stream
    // and return us the first row of the first result.
    let row = stream.into_row().await?;
    
    assert_eq!(Some(-4i32), row.unwrap().get(0));

    Ok(())
}