use actix_web::{ App, HttpServer, middleware::Logger, web };
use actix_files::Files;
use tera::{ Tera };
use webactgree::StaffAccountCredentialRequest;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init();

    let tera: Tera = Tera::new("templates/**/*").expect("Error Initializing Tera");

    let pool = webactgree::db_create_pool().await
    .map_err(|e| {
        eprintln!("DB error: {e}");
        std::io::Error::new(std::io::ErrorKind::Other, "DB init failed")
    })?;
    webactgree::account_option_page(&pool)
    .await
    .expect("Failed to create tables");
    
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(
                web::Data::new(pool.clone())
            )
            .app_data(
                web::Data::new(tera.clone())
            )
            .service(
                Files::new("/assets", "./assets")
            )
            .service(webactgree::delete_detail)
            .service(webactgree::render_detail)
            .service(webactgree::dashboard)
            .service(webactgree::staff_account_creation_page)
            .route(
                "/account-registration-confirmation", 
                web::post()
                .to(StaffAccountCredentialRequest::create_staff_account))
            .service(webactgree::render_regis_page)
    })
    .bind(("127.0.0.1", 2337))?
    .run()
    .await
}
