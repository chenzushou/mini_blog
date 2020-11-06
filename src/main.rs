use actix_web::{App, HttpResponse, HttpServer, get, http::header::ContentType, http::{self, header::ACCESS_CONTROL_ALLOW_ORIGIN}, middleware, web::{self, route}};
mod zhuge;
use std::io::Write;
use env_logger::fmt::Color;
use chrono::Local;
use log::*;
use zhuge::{Zhuge}; 

fn get_score()->HttpResponse{
    let result = zhuge::get_score().unwrap();
    HttpResponse::Ok().header(ACCESS_CONTROL_ALLOW_ORIGIN, "*").json(result)
}

#[actix_web::main]
async fn main()->std::io::Result<()>{

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::builder().format(|buf, record| {
        let mut level_style = buf.style();
        level_style.set_color(Color::Green).set_bold(true);
        let mut time_style=buf.style();
        time_style.set_color(Color::Yellow);
        writeln!(buf, "{} {} : {}",
        time_style.value(Local::now().format("%Y-%m-%d %H:%M:%S")),
        level_style.value(record.level()),
        record.args())
    }).init();



    HttpServer::new(||{
        App::new()
        .wrap(middleware::Logger::default())
        .service(  
        web::scope("/zugescore")
                .route("", web::get().to(get_score))
        )
        .service(actix_files::Files::new("/","./static/").index_file("index.html"))
    }     
    )
    .bind("127.0.0.1:8080")?
    .run()
    .await
} 

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::dev::Service;
    use actix_web::{http, test, web, App};

    #[actix_rt::test]
    async fn test_index() {
        let mut app = test::init_service(
            App::new().service(web::resource("/zugescore").route(web::get().to(get_score))),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/zugescore")
            .to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };

        assert_eq!(response_body, r##"{"point":"100"}"##);

    }
}