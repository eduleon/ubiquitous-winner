#[macro_use]
extern crate actix_web;
extern crate ureq;

use actix_web::{App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    println!("GET: /");
    HttpResponse::Ok().body("Hello world!")
}

#[get("/again")]
async fn again() -> impl Responder {
    println!("GET: /again");
    HttpResponse::Ok().body("Hello world again :B!")
}

#[get("/indecon")]
async fn indecon() -> impl Responder {
    let resp = ureq::get("https://www.indecon.online/")
        .set("X-My-Header", "Secret")
        .call();

    let json = resp.into_json().unwrap();
    println!("json: {:?}", json);

    HttpResponse::Ok().json(json)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Starting actix-web server (port: 5000)");

    HttpServer::new(|| App::new().service(index).service(again).service(indecon))
        .bind("0.0.0.0:5000")?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_test() {
        assert!(true);
    }
}
