use crate::models::my_obj::MyObj;
use actix_web::{get, HttpResponse, Responder};

#[get("/obj")]
pub async fn get_obj() -> impl Responder {
    let obj = MyObj {
        name: "John Doe".to_string(),
        age: 30,
    };
    HttpResponse::Ok().json(obj)
}
