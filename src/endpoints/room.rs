use crate::endpoints::house::HouseContext;
use crate::errors::{Error, Successful, WebResponse};
use crate::wrapper::JsonRoom;
use actix_web::{delete, get, put, web};
use actix_web::{HttpResponse, ResponseError};
use smarthome::Room;

#[get("/rooms")]
async fn get_rooms(ctx: web::Data<HouseContext>) -> WebResponse<web::Json<Vec<JsonRoom>>> {
    let house = ctx.get_context().lock().await;
    let json_rooms = house.get_rooms().into_iter().map(JsonRoom::from).collect();

    Ok(web::Json(json_rooms))
}

#[get("/{room}")]
async fn get_room(
    ctx: web::Data<HouseContext>,
    path: web::Path<String>,
) -> WebResponse<web::Json<JsonRoom>> {
    let house = ctx.get_context().lock().await;
    let room_name = path.into_inner();
    match house.get_room(room_name.as_str()) {
        None => Err(Error::RoomNotFound),
        Some(r) => Ok(web::Json(JsonRoom::from(r))),
    }
}

#[put("/{room}")]
async fn put_room(ctx: web::Data<HouseContext>, path: web::Path<String>) -> HttpResponse {
    let mut house = ctx.get_context().lock().await;
    let room_name = path.into_inner();
    let room = Room::new(room_name.as_str());
    match house.add_room(room) {
        Err(_) => Error::RoomAlreadyExists.error_response(),
        Ok(_) => Successful::ok_response("Done"),
    }
}

#[delete("/{room}")]
async fn del_room(ctx: web::Data<HouseContext>, path: web::Path<String>) -> HttpResponse {
    let mut house = ctx.get_context().lock().await;
    let room_name = path.into_inner();
    match house.del_room(room_name.as_str()) {
        Err(_) => Error::RoomNotFound.error_response(),
        Ok(_) => Successful::ok_response("Done"),
    }
}
