use mysql::*;
use mysql::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct JsonRequest {
    data: String,
}

pub fn insert_data(json: JsonRequest) -> Result<()> {
    let url = std::env::var("DATABASE_URL")
        .map_err(|_| mysql::Error::Other("DATABASE_URL environment variable not set".into()))?;
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;

    // Get table and column names from configuration
    let table_name = std::env::var("DB_TABLE_NAME").unwrap_or_else(|_| "table_name".to_string());
    let column_name = std::env::var("DB_COLUMN_NAME").unwrap_or_else(|_| "column_name".to_string());

    // Validate input data
    if json.data.is_empty() {
        return Err(mysql::Error::Other("Empty data provided".into()));
    }

    conn.exec_drop(
        format!(r"INSERT INTO {} ({}) VALUES (:data)", table_name, column_name),
        params! {
            "data" => json.data,
        },
    )?;

    println!("Data successfully inserted into the database");
    Ok(())
}
