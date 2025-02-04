use super::handlers;
use actix_web::web;

pub fn config(config: &mut web::ServiceConfig) {
    config.service(web::scope("/auth"));
}
