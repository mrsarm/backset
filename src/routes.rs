use actix_web::web;
use crate::health::health_check_handler;
use crate::tenants::api::{create, delete, list, read};
use crate::elements::api::{
    create as elements_create,
    delete as elements_delete,
    read as elements_read,
};

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/health").service(health_check_handler);
    conf.service(scope);
    let scope = web::scope("/tenants")
        .service(create)
        .service(delete)
        .service(list);
    conf.service(scope);
    let scope = web::scope("")
        .service(elements_create)
        .service(elements_delete)
        .service(elements_read)
        .service(read);
    conf.service(scope);
}
