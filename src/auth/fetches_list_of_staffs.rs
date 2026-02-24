use serde::Serialize;
use uuid::Uuid;
use chrono::{ DateTime, Utc };

#[derive(sqlx::FromRow, Serialize)]
pub struct Staff
{
    id: Uuid,
    first_name: String,
    last_name: String,
    email: String,
}

#[derive(sqlx::FromRow, Serialize)]
pub struct StaffDetailResponse
{
    id: Uuid,
    first_name: String,
    last_name: String,
    age: i32,
    username: String,
    email: String,
    password: String,
    jobs: Vec<JobsResponse>,
    is_admin: bool,
    is_active: bool,
    created_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Serialize)]
pub struct StaffDetailRow
{
    id: Uuid,
    first_name: String,
    last_name: String,
    age: i32,
    username: String,
    email: String,
    password: String,
    is_admin: bool,
    is_active: bool,
    created_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Serialize)]
pub struct JobsResponse
{
    occupation: String,
    years_of_experience: i32,
    specific_roles: Vec<SpecificRolesResponse>
}

#[derive(sqlx::FromRow, Serialize)]
pub struct JobsRow
{
    id: Uuid,
    occupation: String,
    years_of_experience: i32,
}

#[derive(sqlx::FromRow, Serialize)]
pub struct SpecificRolesResponse
{
    specific_roles_in_jobs: String,
    staff_dashboard_access: StaffDashboardAccessResponse
}

#[derive(sqlx::FromRow, Serialize)]
pub struct SpecificRolesRow
{
    id: Uuid,
    specific_roles_in_jobs: String,
}

#[derive(Debug, sqlx::Type, Serialize)]
#[sqlx(type_name = "permission_level", rename_all = "UPPERCASE")]
pub enum Permissions
{
    NONE,
    READ,
    WRITE,
    FULL,
}

#[derive(sqlx::FromRow, Serialize)]
pub struct StaffDashboardAccessResponse
{
    managing_post_page: Permissions,
    managing_multi_social_media_post_page: Permissions,
    managing_user_account_page: Permissions,
    managing_user_transaction_page: Permissions,
}

pub async fn fetches_list_of_staffs(pool: &sqlx::PgPool) -> Result<Vec<Staff>, sqlx::Error>
{
    sqlx::query_as::<_, Staff>(
        "SELECT id, first_name, last_name, email FROM staff"
    ).fetch_all(pool)
    .await
}

pub async fn fetches_staff_detail(
    pool: &sqlx::PgPool,
    id: Uuid) -> Result<StaffDetailResponse, sqlx::Error>
{
    //let pool = crate::db_create_pool().await?;

    let staff = sqlx::query_as::<_, StaffDetailRow>(
        "SELECT id, first_name, last_name, age, username, email, password, is_admin, is_active, created_at 
        FROM staff
        WHERE id = $1"
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    let job_rows = sqlx::query_as::<_, JobsRow>(
        "SELECT id, occupation, years_of_experience
        FROM jobs
        WHERE staff_id = $1"
    )
    .bind(id)
    .fetch_all(pool)
    .await?;

    let mut jobs: Vec<JobsResponse> = Vec::new();

        for job_row in job_rows
        {
            let role_rows = sqlx::query_as::<_, SpecificRolesRow>(
                "SELECT id, specific_roles_in_jobs
                FROM specific_roles
                WHERE job_id = $1"
            )
            .bind(job_row.id)
            .fetch_all(pool)
            .await?;

            let mut specific_roles: Vec<SpecificRolesResponse> = Vec::new();

            for role_row in role_rows
            {
                let access = sqlx::query_as::<_, StaffDashboardAccessResponse>(
                    "SELECT managing_post_page,
                                managing_multi_social_media_post_page,
                                managing_user_account_page,
                                managing_user_transaction_page
                        FROM staff_dashboard_access
                        WHERE specific_role_id = $1"
                )
                .bind(role_row.id)
                .fetch_one(pool)
                .await?;

                specific_roles.push(SpecificRolesResponse {
                    specific_roles_in_jobs: role_row.specific_roles_in_jobs,
                    staff_dashboard_access: access,
                });
            }
            jobs.push(JobsResponse {
                occupation: job_row.occupation,
                years_of_experience: job_row.years_of_experience,
                specific_roles,
            });
        }

    Ok(StaffDetailResponse { 
            id: staff.id, 
            first_name: staff.first_name, 
            last_name: staff.last_name, 
            age: staff.age, 
            username: staff.username, 
            email: staff.email, 
            password: staff.password, 
            jobs,
            is_admin: staff.is_admin, 
            is_active: staff.is_active, 
            created_at: staff.created_at, 
        }
    )
}
