use sqlx::postgres::PgPool;
use dotenv::dotenv;
use std::env;

#[derive(Debug, sqlx::FromRow)]
struct DbEntry {
    id: String,
    url: String,
}

pub async fn get_url_by_id(id: &str) -> Result<String, sqlx::Error> {
    dotenv().ok();
    let pool = PgPool::connect(
        &env::var("DATABASE_URL")
        .expect("Please specify database url"))
        .await?;

    let entry = sqlx::query_as!(DbEntry, r#"SELECT * FROM url_shortener WHERE id = $1"#, id)
        .fetch_one(&pool)
        .await?;

    println!("{} -> {}", &entry.id, &entry.url);
    Ok(entry.url)
}

pub async fn save_to_db(id: &str, url: &str) -> Result<(), sqlx::Error> {
    dotenv().ok();
    let pool = PgPool::connect(
        &env::var("DATABASE_URL")
        .expect("Please specify database url"))
        .await?;

    sqlx::query!(r#"INSERT INTO url_shortener VALUES ($1, $2);"#, id, url)
        .execute(&pool)
        .await?;

    Ok(())
}
