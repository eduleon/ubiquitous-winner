extern crate actix_web;
#[macro_use]
extern crate serde_json;

extern crate serde_derive;
use serde_derive::Deserialize;

extern crate ureq;

use ureq::{SerdeMap, SerdeValue};
extern crate chrono;
use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use chrono::NaiveDateTime;
use std::collections::HashMap;
use std::env;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[derive(Deserialize)]
pub struct SMARequest {
   indicator: String
}

#[get("/")]
async fn index() -> impl Responder {
    println!("GET: /");
    HttpResponse::Ok().body("Hello world!")
}

// https://www.alphavantage.co/query
// ?function=SMA&symbol=IBM&interval=weekly&time_period=10&series_type=open&apikey=demo
#[get("/sma")]
async fn get_sma(web::Query(request): web::Query<SMARequest>) -> impl Responder {
    println!("GET: /sma?indicator={}", request.indicator);
    HttpResponse::Ok().body("Hello world again :B!")
}

#[get("/last")]
async fn indecon() -> impl Responder {
    let resp = ureq::get("https://www.indecon.online/last")
        .set("X-My-Header", "Secret")
        .call();

    let json = resp.into_json().unwrap();
    println!("json: {:?}", json);

    HttpResponse::Ok().json(json)
}

#[get("/trend/{indicator}")]
async fn get_indicator_trend(indicator: web::Path<String>) -> impl Responder {
    let api_uri = format!("https://www.indecon.online/values/{}", indicator);
    let resp = ureq::get(&api_uri).call();

    let json = resp.into_json().unwrap();
    //println!("json: {:?}", json);

    HttpResponse::Ok().json(json)
}

fn get_sma_from_timeseries(
    resp_map: &SerdeMap<String, SerdeValue>,
) -> HashMap<std::string::String, f64> {
    let mut prefix_sum = 0.0;
    let mut counter = 1.0;
    let mut new_map = HashMap::new();
    for (key, value) in resp_map {
        prefix_sum += value.as_f64().unwrap();
        let sma = prefix_sum / counter;
        //println!("key: {}, value: {}, counter {}, prefix_sum: {}, sma: {}", key, value, counter, prefix_sum, sma);
        counter += 1.0;
        let date_time = NaiveDateTime::from_timestamp(key.parse::<i64>().unwrap(), 0);
        let date = date_time.format("%Y-%m-%d").to_string();
        new_map.insert(date, sma);
    }
    new_map
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=debug");
    //env_logger::init();

    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        println!("Starting scheduler");
        loop {
            println!("Periodic call");
            thread::sleep(Duration::from_secs(3));

            let response = ureq::get("https://www.indecon.online/values/cobre")
                .set("X-My-Header", "Secret")
                .call();
            let response_json = response.into_json().unwrap();

            /*
            println!("Cobre: {}", json["cobre"]);
            println!("Cobre: {}", json["cobre"]["name"]);
            println!("Values: {}", json["values"]);
            */
            let response_map = response_json["values"].as_object().unwrap();
            let sma_map = get_sma_from_timeseries(response_map);

            sender.send(sma_map).unwrap();
        }
    });

    //let received = receiver.recv().unwrap();
    //println!("Got: {}", received);
    /*
    for received in receiver {
        let asd = json!(received);
        //println!("Got: {}", asd);
        println!("{}", serde_json::to_string_pretty(&asd).unwrap());
    }
    */

    println!("Starting actix-web server (port: 5000)");
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(index)
            .service(indecon)
            .service(get_sma)
            .service(get_indicator_trend)
    })
    .bind("0.0.0.0:5000")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    //    use super::*;

    #[test]
    fn sample_test() {
        assert!(true);
    }
}
