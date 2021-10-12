use odbc_iter::{Odbc, ValueRow};

fn main() {
    // Connect to database using connection string
    let connection_string =
        "Driver={SQL Server};Server=.;Database=TmsRobots;Uid=plugin;Pwd=Y0djcnm;";
    //std::env::var("DB_CONNECTION_STRING").expect("DB_CONNECTION_STRING environment not set");
    let mut connection = Odbc::connect(&connection_string).expect("failed to connect to database");

    // Handle statically guards access to connection and provides query functionality
    let mut db = connection.handle();

    // Get single row single column value
    println!(
        "{}",
        db.query::<String>("SELECT 'hello world'")
            .expect("failed to run query")
            .single()
            .expect("failed to fetch row")
    );

    // Iterate rows with single column
    for row in db
        .query::<String>("SELECT 'hello world' UNION SELECT 'foo bar'")
        .expect("failed to run query")
    {
        println!("{}", row.expect("failed to fetch row"))
    }
    // Prints:
    // hello world
    // foo bar

    // Iterate rows with multiple columns
    for row in db
        .query::<(String, i8)>(
            "SELECT 'hello world', CAST(24 AS TINYINT) UNION SELECT 'foo bar', CAST(32 AS TINYINT)",
        )
        .expect("failed to run query")
    {
        let (string, number) = row.expect("failed to fetch row");
        println!("{} {}", string, number);
    }
    // Prints:
    // hello world 24
    // foo bar 32

    // Iterate rows with dynamically typed values using `ValueRow` type that can represent
    // any row
    for row in db
        .query::<ValueRow>("SELECT 'hello world', 24 UNION SELECT 'foo bar', 32")
        .expect("failed to run query")
    {
        println!("{:?}", row.expect("failed to fetch row"))
    }
    // Prints:
    // [Some(String("hello world")), Some(Tinyint(24))]
    // [Some(String("foo bar")), Some(Tinyint(32))]

    for row in db
        .query::<ValueRow>("SELECT top 2 * from [dbo].[DataExport30]")
        .expect("failed to run query")
    {
        println!("{:?}", row.expect("failed to fetch row"))
    }
}
