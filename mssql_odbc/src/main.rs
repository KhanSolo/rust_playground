use odbc::*;
use odbc::ResultSetState::{NoData, Data};

fn connect() -> std::result::Result<(), DiagnosticRecord> {

    let env = create_environment_v3().map_err(|e| e.unwrap())?;

    let buffer = String::from("Driver={SQL Server};Server=.;Database=BgtTelegramService;Uid=plugin;Pwd=Y0djcnm;ClientCharset=UTF-8;ServerCharset=UTF-8;");
    //println!("Please enter connection string: ");
    // Driver={SQL Server};Server=.;Database=TmsRobots;Uid=plugin;Pwd=Y0djcnm;
    //io::stdin().read_line(&mut buffer).unwrap();

    let conn = env.connect_with_connection_string(&buffer)?;
    let sql_text = String::from("SELECT * FROM [dbo].[Contact] order by 1");
    execute_statement(&conn, &sql_text)
}

fn execute_statement<T: odbc::odbc_safe::AutocommitMode>(conn: &Connection<T>, sql_text: &str) -> Result<()> {

    let stmt = Statement::with_parent(conn)?;
    let result_set_state = stmt.exec_direct(&sql_text)?;

    match result_set_state {
        Data(mut stmt) => {
            let desc = stmt.describe_col(1u16)?;
            println!("{}", desc.name);

            let cols = stmt.num_result_cols()?;
            while let Some(mut cursor) = stmt.fetch()? {

                for i in 1..=cols {
                    match cursor.get_data::<&str>(i as u16)? {
                        Some(val) => {
                            print!(" {}", val);
                        },
                        None => print!(" <null>"),
                    }
                }
                println!("");
            }
        }
        NoData(_) => println!("Query executed, no data returned"),
    }

    Ok(())
}

fn main() {

    match connect() {
        Ok(()) => println!("Success"),
        Err(diag) => println!("Error: {}", diag),
    }
}
