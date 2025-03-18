use mysql::*;
use mysql::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct JsonRequest {
    data: String,
}

pub fn insert_data(json: JsonRequest) -> Result<()> {
    let url = "mysql://username:password@localhost:3306/database_name";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;

    conn.exec_drop(
        r"INSERT INTO table_name (column_name) VALUES (:data)",
        params! {
            "data" => json.data,
        },
    )?;

    Ok(())
}
