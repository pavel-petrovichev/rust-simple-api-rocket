#[warn(unused_mut)]

use rocket::{get, http::Status, serde::json::Json};
use serde::Serialize;

#[macro_use]
extern crate rocket;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[get("/healthchecker")]
pub async fn health_checker_handler() -> Result<Json<GenericResponse>, Status> {
    const MESSAGE: &str = "Build Simple CRUD API with Rust and Rocket";

    let response_json = GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    Ok(Json(response_json))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/api",
            routes![
                health_checker_handler,
            ])
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::{Status};

    #[test]
    fn healthchecker_status() {
        let client = Client::tracked(rocket())
            .expect("valid rocket instance");
        let response = client
            .get(uri!("/api", super::health_checker_handler))
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn healthchecker_body() {
        let client = Client::tracked(rocket())
            .expect("valid rocket instance");
        let response = client
            .get(uri!("/api", super::health_checker_handler))
            .dispatch();

        //assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().unwrap(), 
            r#"{"status":"success","message":"Build Simple CRUD API with Rust and Rocket"}"#);
    }
}
