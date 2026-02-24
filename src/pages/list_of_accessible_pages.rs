use actix_web::{ HttpResponse, Responder, error::ErrorInternalServerError, get, web, post };
use tera::{ Context, Tera };
use uuid::Uuid;

// DASHBOARD PAGE
#[get("/")]
async fn dashboard(tmpl: web::Data<Tera>) -> impl Responder
{
    let mut ctx = Context::new();
    ctx.insert("page", "Dashboard");

    let rendered = tmpl.render("index.html", &ctx).map_err(actix_web::error::ErrorInternalServerError).unwrap();

    HttpResponse::Ok()
    .content_type("text/html")
    .body(rendered)
}

pub fn cus_render(tmpl: &Tera, name: &str, ctx: Context) -> HttpResponse
{
    match tmpl.render(name, &ctx) {
        Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

// THE REGISTRATION PAGE
#[get("/register-account")]
pub async fn staff_account_creation_page(tmpl: web::Data<Tera>) -> HttpResponse
{
    let mut ctx = Context::new();
    ctx.insert("page", "Staff Registration");
    cus_render(&tmpl, "page/account-registration.html", ctx)
}

// ACCESSING REGISTRATION PAGE
#[get("/account-registration-confirmation")]
pub async fn render_regis_page(pool: web::Data<sqlx::PgPool>, tmpl: web::Data<Tera>) -> Result<HttpResponse, actix_web::Error> 
{
    let staffs = crate::fetches_list_of_staffs(&pool)
        .await
        .map_err(ErrorInternalServerError)?;


    let mut ctx: Context = Context::new();
    ctx.insert("staffs", &staffs);

    Ok(cus_render(&tmpl, "page/regis-process.html", ctx))
}

#[get("/account-registration-confirmation/{id}")]
pub async fn render_detail(
    pool: web::Data<sqlx::PgPool>, 
    tmpl: web::Data<Tera>, 
    path: web::Path<Uuid>) -> Result<HttpResponse, actix_web::Error>
{
    let id = path.into_inner();

    let staff_detail = crate::fetches_staff_detail(&pool, id)
        .await
        .map_err(ErrorInternalServerError)?;

    let mut ctx: Context = Context::new();
    ctx.insert("staff", &staff_detail);

    Ok(cus_render(&tmpl, "page/detail-account-page.html", ctx))
}

#[post("/account-registration-confirmation/delete/{id}")]
pub async fn delete_detail(
    pool: web::Data<sqlx::PgPool>, 
    path: web::Path<Uuid>) -> Result<HttpResponse, actix_web::Error>
{
    let id = path.into_inner();

    let staff_detail = crate::delete_staff_detail(pool.get_ref(), id)
        .await
        .map_err(ErrorInternalServerError)?;

    let mut ctx: Context = Context::new();
    ctx.insert("staff", &staff_detail);

    Ok(HttpResponse::SeeOther()
        .append_header(("Location", "/account-registration-confirmation"))
        .finish())
}