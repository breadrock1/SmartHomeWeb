use crate::endpoints::house::HouseContext;
use crate::errors::{Error, Successful, WebResponse};
use crate::wrapper::JsonDevice;
use actix_web::{delete, get, put, web};
use actix_web::{HttpResponse, ResponseError};
use smarthome::sockets::smart_sockets::SocketType;
use smarthome::{SmartSocket, SmartThermometer, SocketTrait};

#[get("/{room}")]
async fn get_devices(
    ctx: web::Data<HouseContext>,
    path: web::Path<String>,
) -> WebResponse<web::Json<Vec<JsonDevice>>> {
    let house = ctx.get_context().lock().await;
    let room_name = path.as_str();
    match house.get_room(room_name) {
        None => Err(Error::DeviceNotFound),
        Some(room) => {
            let devices: Vec<JsonDevice> = room
                .get_devices()
                .into_iter()
                .map(JsonDevice::from)
                .collect();

            Ok(web::Json(devices))
        }
    }
}

#[get("/{room}/{device}")]
async fn get_device(
    ctx: web::Data<HouseContext>,
    path: web::Path<(String, String)>,
) -> WebResponse<web::Json<JsonDevice>> {
    let house = ctx.get_context().lock().await;
    let (room_name, dev_name) = path.into_inner();
    let room = house
        .get_room(room_name.as_str())
        .ok_or(Error::DeviceNotFound)
        .unwrap();

    match room.get_device(dev_name.as_str()) {
        None => Err(Error::DeviceNotFound),
        Some(d) => Ok(web::Json(JsonDevice::from(d))),
    }
}

#[put("/{room}/{device}/{type}")]
async fn put_device(
    ctx: web::Data<HouseContext>,
    path: web::Path<(String, String, String)>,
) -> HttpResponse {
    let mut house = ctx.get_context().lock().await;
    let (room_name, dev_name, dev_type) = path.into_inner();

    let device = match dev_type.as_str() {
        "socket" => SocketType::from(SmartSocket::new(dev_name.as_str())),
        "thermometer" => SocketType::from(SmartThermometer::new(dev_name.as_str())),
        _ => SocketType::from(SmartSocket::new(dev_name.as_str())),
    };

    let room = house
        .get_room_mut(room_name.as_str())
        .ok_or(Error::DeviceNotFound)
        .unwrap();

    match room.add_device(device) {
        Err(_) => Error::DeviceAlreadyExists.error_response(),
        Ok(_) => Successful::ok_response("Done"),
    }
}

#[delete("/{room}/{device}")]
async fn del_device(
    ctx: web::Data<HouseContext>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let mut house = ctx.get_context().lock().await;
    let (room_name, dev_name) = path.into_inner();
    let room = house
        .get_room_mut(room_name.as_str())
        .ok_or(Error::RoomNotFound)
        .unwrap();

    match room.del_device(dev_name.as_str()) {
        Err(_) => Error::DeviceNotFound.error_response(),
        Ok(_) => Successful::ok_response("Done"),
    }
}
