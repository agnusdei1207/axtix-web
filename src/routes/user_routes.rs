use super::{handlers, middlewares};
use actix_web::{
    middleware::{self, from_fn},
    web,
};

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/user")
            .wrap(from_fn(middlewares::auth_middleware::check_auth_middleware))
            .service(handlers::user_handler::user)
            .service(handlers::user_handler::update_user),
    );
}
