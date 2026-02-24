use uuid::Uuid;

pub async fn delete_staff_detail(
    pool: &sqlx::PgPool,
    id: Uuid) -> Result<u64, sqlx::Error>
{
    let result = sqlx::query(
        "DELETE FROM staff WHERE id = $1"
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}