use sqlx::PgPool;

pub async fn create_user(
    pool: &PgPool,
    email: &str,
    password: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let hashed_password = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;

    // let query = r#"
    // INSERT INTO users(email, password)
    // VALUES($1, $2)
    // "#;
    //
    // let result = sqlx::query(query).bind(email).bind(hashed_password).execute(pool).await;

    sqlx::query!(
        "INSERT INTO users(email, password) VALUES($1, $2)",
        email,
        hashed_password
    )
    .execute(pool)
    .await?;
    Ok(())
}
