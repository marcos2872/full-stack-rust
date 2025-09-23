use crate::data::errors::DataError;
use sqlx::PgPool;

pub async fn create_user(
    pool: &PgPool,
    email: &str,
    password: &str,
) -> Result<(), DataError> {
    let hashed_password = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;

    sqlx::query!(
        "INSERT INTO users(email, password) VALUES($1, $2)",
        email,
        hashed_password
    )
    .execute(pool)
    .await?;

    Ok(())
}