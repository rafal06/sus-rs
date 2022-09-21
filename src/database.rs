use sqlx::postgres::PgPool;
use dotenv::dotenv;
use std::env;

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
