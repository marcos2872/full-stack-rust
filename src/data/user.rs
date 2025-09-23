use crate::data::errors::DataError;
use sqlx::PgPool;

pub async fn create_user(pool: &PgPool, email: &str, password: &str) -> Result<(), DataError> {
    let hashed_password = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;

    sqlx::query!(
        "INSERT INTO users(email, password) VALUES($1, $2)",
        email,
        hashed_password
    )
    .execute(pool)
    .await
    .map_err(|err| match err {
        sqlx::Error::Database(e) => {
            if e.constraint() == Some("users_email_key") {
                DataError::FailedQuery("This email address is already used".to_string())
            } else {
                DataError::Internal(e.to_string())
            }
        }
        e => DataError::Query(e),
    })?;

    Ok(())
}
