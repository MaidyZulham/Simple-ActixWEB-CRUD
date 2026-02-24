use sqlx::{ Pool, Postgres };

pub async fn account_option_page(_pool: &Pool<Postgres>) -> Result<(), sqlx::Error>
{
    let pool: Pool<Postgres> = crate::db_create_pool().await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS staff(
        id UUID NOT NULL PRIMARY KEY,
        first_name VARCHAR(20) NOT NULL,
        last_name VARCHAR(20) NOT NULL,
        age SMALLINT NOT NULL CHECK (age >= 0),
        username TEXT UNIQUE NOT NULL,
        email TEXT UNIQUE NOT NULL,
        password VARCHAR(255) NOT NULL,
        is_admin BOOLEAN DEFAULT FALSE,
        is_active BOOLEAN DEFAULT TRUE,
        created_at TIMESTAMP WITH TIME ZONE NOT NULL
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS jobs(
        id UUID PRIMARY KEY,
        staff_id UUID REFERENCES staff(id) ON DELETE CASCADE,
        occupation TEXT NOT NULL,
        years_of_experience INTEGER NOT NULL
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS specific_roles(
        id UUID PRIMARY KEY,
        job_id UUID REFERENCES jobs(id) ON DELETE CASCADE,
        specific_roles_in_jobs TEXT NOT NULL
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "DO $$
        BEGIN
        CREATE TYPE permission_level AS ENUM (
            'NONE',
            'READ',
            'WRITE',
            'FULL'
        );
        EXCEPTION
            WHEN duplicate_object THEN null;
        END $$;"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS staff_dashboard_access(
        id UUID PRIMARY KEY,
        specific_role_id UUID REFERENCES specific_roles(id) ON DELETE CASCADE,

        managing_post_page permission_level NOT NULL,
        managing_multi_social_media_post_page permission_level NOT NULL,
        managing_user_account_page permission_level NOT NULL,
        managing_user_transaction_page permission_level NOT NULL
        )"
    )
    .execute(&pool)
    .await?;

    println!("TABLES CREATED SUCCESSFULLY!");

    Ok(())
}