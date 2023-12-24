use crate::elements::api::{
    create as elements_create,
    delete as elements_delete,
    list as elements_list,
    read as elements_read,
    put as elements_put,
};
use crate::health::health_check_handler;
use crate::tenants::api::{create, delete, list, read, put};
use actix_web::web;

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/health").service(health_check_handler);
    conf.service(scope);

    let scope = web::scope("/tenants")
        .service(create)
        .service(delete)
        .service(list)
        .service(read)
        .service(put);
    conf.service(scope);

    // "/{tenant}" and "/{tenant}/{id}"
    let scope = web::scope("")
        .service(elements_create)
        .service(elements_delete)
        .service(elements_list)
        .service(elements_read)
        .service(elements_put);
    conf.service(scope);
}
