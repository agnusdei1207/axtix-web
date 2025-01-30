mod home;
mod obj;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(home::index);
    cfg.service(obj::get_obj);
}
