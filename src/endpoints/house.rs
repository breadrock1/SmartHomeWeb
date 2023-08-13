use actix_web::{get, web};
use smarthome::SmartHouse;
use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::errors::WebResponse;
use crate::wrapper::JsonSmartHouse;

#[derive(Default, Clone)]
pub struct HouseContext {
    context: Arc<Mutex<SmartHouse>>,
}

impl HouseContext {
    pub fn _new(address: &str) -> Option<Self> {
        let house = SmartHouse::new(address);
        let house = Mutex::new(house);
        let house = Arc::new(house);
        Some(HouseContext { context: house })
    }

    pub fn get_context(&self) -> &Arc<Mutex<SmartHouse>> {
        &self.context
    }
}

#[get("/")]
async fn get_house(ctx: web::Data<HouseContext>) -> WebResponse<web::Json<JsonSmartHouse>> {
    let house = ctx.get_context().lock().await;
    let house_object = house.deref();
    Ok(web::Json(JsonSmartHouse::from(house_object)))
}
