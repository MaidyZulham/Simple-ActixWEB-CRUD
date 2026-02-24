use actix_web::{ HttpResponse, web };
use serde::{ Serialize, Deserialize };
use uuid::Uuid;
use chrono::{ DateTime, Utc };

fn datetime_gen() -> DateTime<Utc>
{
    Utc::now()
}

// This is the separated request
#[derive(Debug, Serialize, Deserialize)]
pub struct StaffAccountCredentialRequest
{
    first_name: String,
    last_name: String,
    age: i32,
    username: String,
    email: String,
    password: String,
    jobs: Vec<JobsRequest>,
    is_admin: bool,
    is_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JobsRequest
{
    occupation: String,
    years_of_experience: i32,
    specific_roles: Vec<SpecificRolesRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecificRolesRequest
{
    specific_roles_in_jobs: String,
    staff_dashboard_access: StaffDashboardAccessRequest,
}

#[derive(Debug, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "permission_level")]
pub enum Permissions
{
    NONE,
    READ,
    WRITE,
    FULL,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StaffDashboardAccessRequest
{
    managing_post_page: Permissions,
    managing_multi_social_media_post_page: Permissions,
    managing_user_account_page: Permissions,
    managing_user_transaction_page: Permissions,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T>
{
    message: String,
    data: T,
}

// REQUSET & RESPONSE OF REGISTRATION SECTION
// REGISTRATION DATA INSERTION
impl StaffAccountCredentialRequest {
    pub async fn create_staff_account(pool: web::Data<sqlx::PgPool>, payload: web::Json<StaffAccountCredentialRequest>) -> Result<HttpResponse, actix_web::Error>
    {
        let reqs: StaffAccountCredentialRequest = payload.into_inner();

        let mut tx = (**pool).begin().await.map_err(actix_web::error::ErrorInternalServerError)?;

        let uuid_of_staff = Uuid::new_v4();
        let datetime_generator = datetime_gen();

        sqlx::query(
            r#"
            INSERT INTO staff(
            id, first_name, last_name, age, username, email, password, is_admin, is_active, created_at
            )
            VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#
        )
        .bind(uuid_of_staff)
        .bind(&reqs.first_name)
        .bind(&reqs.last_name)
        .bind(&reqs.age)
        .bind(&reqs.username)
        .bind(&reqs.email)
        .bind(&reqs.password)
        .bind(&reqs.is_admin)
        .bind(&reqs.is_active)
        .bind(datetime_generator)
        .execute(&mut *tx)
        .await.map_err(actix_web::error::ErrorInternalServerError)?;

        for job in &reqs.jobs
        {
            let job_id = Uuid::new_v4();

            sqlx::query(
                r#"
                INSERT INTO jobs(
                id, staff_id, occupation, years_of_experience
                )
                VALUES($1, $2, $3, $4)
                "#
            )
            .bind(job_id)
            .bind(uuid_of_staff)
            .bind(&job.occupation)
            .bind(job.years_of_experience)
            .execute(&mut *tx)
            .await.map_err(actix_web::error::ErrorInternalServerError)?;

            for role in &job.specific_roles
            {
                let role_id = Uuid::new_v4();

                sqlx::query(
                    r#"INSERT INTO specific_roles(
                    id, job_id, specific_roles_in_jobs
                    )
                    VALUES($1, $2, $3)"#
                )
                .bind(role_id)
                .bind(job_id)
                .bind(&role.specific_roles_in_jobs)
                .execute(&mut *tx)
                .await.map_err(actix_web::error::ErrorInternalServerError)?;

                let access = &role.staff_dashboard_access;

                let access_id = Uuid::new_v4();

                sqlx::query(
                    r#"INSERT INTO staff_dashboard_access(
                        id,
                        specific_role_id,
                        managing_post_page,
                        managing_multi_social_media_post_page,
                        managing_user_account_page,
                        managing_user_transaction_page
                    )
                    VALUES($1 ,$2, $3, $4, $5, $6)
                    "#
                )
                .bind(access_id)
                .bind(role_id)
                .bind(&access.managing_post_page)
                .bind(&access.managing_multi_social_media_post_page)
                .bind(&access.managing_user_account_page)
                .bind(&access.managing_user_transaction_page)
                .execute(&mut *tx)
                .await.map_err(actix_web::error::ErrorInternalServerError)?;
            }
        }

        tx.commit().await.map_err(actix_web::error::ErrorInternalServerError)?;

        println!("{:#?}", reqs);

        Ok(HttpResponse::Created().json(ApiResponse{
            message: "Staff Account Created Successfully!".to_string(),
            data: reqs,
        }))
    }
}

// fn readable_datetime_gen() -> String
// {
//     let local_datetime: DateTime<Local> = Local::now();
//     let datetime: DateTime<Utc> = local_datetime.with_timezone(&Utc);
//     let readable_timestamp = datetime.format("%d/%m/%Y - %H:%M:%S").to_string();
//     readable_timestamp
// }