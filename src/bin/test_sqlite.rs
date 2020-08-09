use sqlx_core::connection::Connect;

#[async_std::main]
async fn main() {
    match sqlx::sqlite::SqliteConnection::connect("sqlite://crawler.sqlite3.db").await {
        Ok(ok) => {
            println!("connect ok");
        },
        Err(e) => {
            println!("connect failed:{}\n", e);
        }
    }
}