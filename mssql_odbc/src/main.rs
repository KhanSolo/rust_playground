use odbc::*;
use odbc::ResultSetState::{NoData, Data};
use std::io;

fn execute_statement<T: odbc::odbc_safe::AutocommitMode>(conn: &Connection<T>) -> Result<()> {
    let stmt = Statement::with_parent(conn)?;

    let mut sql_text = String::new();
    println!("Please enter SQL statement string:");
    io::stdin().read_line(&mut sql_text).unwrap();

    match stmt.exec_direct(&sql_text)? {
        
    }
}

fn main() {
    println!("Hello, world!");
}
