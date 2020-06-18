extern crate actix_web;
#[macro_use]
extern crate serde_json;

use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};

use std::collections::HashMap;
use std::env;
use std::thread;

mod indicators;
use indicators::init;

#[get("/")]
async fn index() -> impl Responder {
    println!("GET: /");
    HttpResponse::Ok()
        .body("Hello world! Get the Simple Moving Average (SMA) of your favourites commodities")
}

// sample: ?function=SMA&symbol=IBM&interval=weekly&time_period=10&series_type=open&apikey=demo
// TODO validations
#[get("/sma")]
async fn get_sma(query: web::Query<HashMap<String, String>>) -> impl Responder {
    let indicator = query.get("indicator").unwrap();
    let period = query.get("period").unwrap();
    println!("GET: /sma?indicator={}&period={}", indicator, period);
    let sma_map = indicators::get_sma(indicator, period.parse::<usize>().unwrap());

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
