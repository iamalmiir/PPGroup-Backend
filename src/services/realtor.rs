use actix_web::{delete, get, post, web, HttpResponse, Responder};
use sea_orm::DatabaseConnection;

use crate::actors::realtor::Realtor;

#[derive(Debug, serde::Deserialize)]
pub struct PaginationQuery {
    page: Option<u64>,
    page_size: Option<u64>,
}

#[derive(Debug, serde::Deserialize)]
pub struct RealtorParams {
    email: String,
}

#[get("/get")]
pub async fn get_realtors(
    db: web::Data<DatabaseConnection>,
    query: web::Query<PaginationQuery>,
) -> impl Responder {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);

    match Realtor::fetch_all(db.as_ref(), page, page_size).await {
        Ok(realtors) => HttpResponse::Ok().json(realtors),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Adds a new realtor to the database.
///
/// This function is an endpoint for a POST request at the path "/realtors/add".
///
/// # Arguments
///
/// * `db` - A shared reference to a `DatabaseConnection` wrapped in `web::Data`. This is the database connection.
/// * `form` - A JSON payload that is expected to be of the `Realtor` type, wrapped in `web::Json`. This is the data of the realtor to be added.
///
/// # Returns
///
/// Returns an object that implements the `Responder` trait. If the realtor is successfully added to the database, it returns an HTTP response with a status code of 200 (OK) and the newly created realtor in the response body as JSON. If there is an error adding the realtor to the database, it returns an HTTP response with a status code of 500 (Internal Server Error).
///
/// # Errors
///
/// Returns `HttpResponse::InternalServerError` if there is an error adding the realtor to the database.
#[post("/add")]
pub async fn create_realtor(
    db: web::Data<DatabaseConnection>,
    form: web::Json<Realtor>,
) -> impl Responder {
    match Realtor::add_realtor(db.as_ref(), form.into_inner()).await {
        Ok(realtor) => HttpResponse::Ok().json(realtor),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[delete("/delete")]
pub async fn delete_realtor_by_email(
    db: web::Data<DatabaseConnection>,
    query: web::Query<RealtorParams>,
) -> impl Responder {
    let email = query.email.to_string();
    match Realtor::delete_realtor_by_email(db.as_ref(), email.to_string()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn realtors_services() -> actix_web::Scope {
    web::scope("/realtors")
        .service(get_realtors)
        .service(create_realtor)
        .service(delete_realtor_by_email)
}
