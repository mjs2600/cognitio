use std::env;
use std::error::Error;

use sqlx::{Pool, Sqlite, SqlitePool};

#[derive(Debug, Copy)]
struct Book {
    id: i64,
    title: String,
    isbn: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pool = gen_pool().await?;
    sqlx::query!("INSERT INTO books (title) VALUES ('Alice in Wonderland');")
        .execute(&pool)
        .await?;
    let books = sqlx::query_as!(Book, "SELECT * FROM books;")
        .fetch_all(&pool)
        .await?;

    println!("Books {:?}", books);
    let book = &books[0];
    println!("{:?}: {:?} {:?}", book.id, book.title, book.isbn);
    Ok(())
}

async fn gen_pool() -> Result<Pool<Sqlite>, Box<dyn Error>> {
    let db_address = env::var("DATABASE_URL")?;
    let pool = SqlitePool::connect(&db_address).await?;
    Ok(pool)
}
