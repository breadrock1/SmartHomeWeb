use crate::endpoints::device::{del_device, get_device, get_devices, put_device};
use crate::endpoints::house::{get_house, HouseContext};
use crate::endpoints::room::{del_room, get_room, get_rooms, put_room};
use actix_web::{web, App, HttpServer, Scope};

mod endpoints;
mod errors;
mod wrapper;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let ctx = HouseContext::default();
    HttpServer::new(move || {
        let ctx = ctx.clone();
        App::new()
            .app_data(web::Data::new(ctx))
            .service(build_service())
    })
    .bind(("127.0.0.1", 45678))?
    .run()
    .await
}

fn build_service() -> Scope {
    web::scope("/home")
        .service(get_house)
        .service(get_rooms)
        .service(get_room)
        .service(put_room)
        .service(del_room)
        .service(get_devices)
        .service(get_device)
        .service(put_device)
        .service(del_device)
}
