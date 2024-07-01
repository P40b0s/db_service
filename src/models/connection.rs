use std::{path::Path, sync::Arc, time::Duration};
use anyhow::{anyhow, Result};
use once_cell::sync::OnceCell;
use sqlx::{sqlite::{SqliteConnectOptions, SqlitePoolOptions}, Connection, SqliteConnection, SqlitePool};


pub static POOL: once_cell::sync::OnceCell<Arc<SqlitePool>> = OnceCell::new();
pub async fn new_connection<P: AsRef<Path>>(db_name: P) -> Result<()>
{
    let local_path = Path::new(&std::env::current_dir().unwrap()).join([db_name.as_ref().to_str().unwrap(), ".", "sq3"].concat());
    if !local_path.exists()
    {
        std::fs::File::create(&local_path).map_err(|_| anyhow!("Ошибка создания файла базы данных!"))?;
    }
    let options = SqliteConnectOptions::new()
    .filename(local_path)
    .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);

    let pool = SqlitePoolOptions::new()
    .max_connections(5)
    .acquire_timeout(Duration::from_secs(3))
    .connect_with(options)
    .await?;
    POOL.set(Arc::new(pool));
    Ok(())
}
// pub async fn get_connection(db_name: &str) -> Result<SqliteConnection> 
// {
//     let local_path = Path::new(&std::env::current_dir().unwrap()).join([db_name, ".", "sq3"].concat());
//     if !local_path.exists()
//     {
//         std::fs::File::create(&local_path).map_err(|_| anyhow!("Ошибка создания файла базы данных!"))?;
//     }
//     Ok(SqliteConnection::connect(&local_path.display().to_string()).await?)
// }