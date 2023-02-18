use actix_web::{get,post,web,App,HttpResponse,HttpServer,Responder};
use reqwest::Error;

// Function to send POST request to ESP32 to turn on/off the relay
async fn set_relay_state(state: bool) -> Result<String, Error> {
    let url = format!("http://192.168.1.82:80/{state}", state = if state {"on"} else {"off"});
    let res = reqwest::get(&url).await?.text().await?;
    Ok(res)
}

// Handler function for turning on the relay
#[get("/on")]
async fn turn_relay_on() -> impl Responder {
    match set_relay_state(true).await {
        Ok(_) => HttpResponse::Ok().body("Relay turned on"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to turn on relay"),
    }
}

// Handler function for turning off the relay
#[get("/off")]
async fn turn_relay_off() -> impl Responder {
    match set_relay_state(false).await {
        Ok(_) => HttpResponse::Ok().body("Relay turned off"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to turn off relay"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(turn_relay_on)
            .service(turn_relay_off)
    })
    .bind(("192.168.1.77",7878))?
    .run()
    .await
}