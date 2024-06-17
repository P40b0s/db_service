use std::path::Path;
use anyhow::{anyhow, Result};
use sqlx::{Connection, SqliteConnection};

pub async fn get_connection(db_name: &str) -> Result<SqliteConnection> 
{
    let local_path = Path::new(&std::env::current_dir().unwrap()).join([db_name, ".", "sq3"].concat());
    if !local_path.exists()
    {
        std::fs::File::create(&local_path).map_err(|_| anyhow!("Ошибка создания файла базы данных!"))?;
    }
    Ok(SqliteConnection::connect(&local_path.display().to_string()).await?)
}