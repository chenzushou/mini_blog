use actix_web::{get,web,App,HttpServer,Responder,HttpResponse,Result,HttpRequest,middleware,Error as AWError};
mod zhuge;
use std::io::Write;
use env_logger::fmt::Color;
use chrono::Local;
use zhuge::{Zhuge};

async fn get_score(data: web::Data<Zhuge>)->impl Responder{
    let result = zhuge::get_score().unwrap();
    HttpResponse::Ok().json(result)
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



    HttpServer::new(move||
        App::new()
        .wrap(middleware::Logger::default())
        .service(actix_files::Files::new("/","./static/").index_file("index.html"))
        .service(web::resource("/zugescore").route(web::post().to(get_score)))
    )
    .bind("127.0.0.1:8080")?
    .run()
    .await
}   