extern crate actix_web;
#[macro_use]
extern crate serde_json;

//extern crate serde_derive;
use serde_derive::{Serialize, Deserialize};

use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};

use std::env;
use std::thread;
use std::collections::HashMap;

mod indicators;
use indicators::init;

#[derive(Deserialize)]
pub struct SMARequest {
    indicator: String,
    interval: String,
    period: u8,
}
/*
#[derive(Serialize, Deserialize, Default)]
struct MyStruct {
    #[serde(serialize_with = "ordered_map")]
    map: HashMap<String, String>,
}
*/

#[get("/")]
async fn index() -> impl Responder {
    println!("GET: /");
    HttpResponse::Ok().body("Hello world! Get the Simple Moving Average of your favorits commodities")
}

// https://www.alphavantage.co/query
// ?function=SMA&symbol=IBM&interval=weekly&time_period=10&series_type=open&apikey=demo
#[get("/sma")]
async fn get_sma(query : web::Query<HashMap<String, String>>) -> impl Responder {
    let indicator = query.get("indicator").unwrap();
    println!("GET: /sma?indicator={}", indicator);
    let sma_map = indicators::get_sma(indicator);
    HttpResponse::Ok().json(sma_map)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=debug");
    thread::spawn(move || {
        println!("Starting indicators loader in parallel");
        init();
    });

    println!("Starting actix-web server (PORT: 5000)");
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(index)
            .service(get_sma)
    })
    .bind("0.0.0.0:5000")?
    .run()
    .await
}
