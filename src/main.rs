use actix_web::{get,web,App,HttpServer,Responder,HttpResponse,Result,HttpRequest,middleware};
mod zhuge;
use std::io::Write;
use futures::future::join_all;
use r2d2_sqlite::{self, SqliteConnectionManager};
use env_logger::fmt::Color;
use chrono::Local;
use zhuge::{Zhuge,action,Pool};
async fn get_score(db:web::Data<Pool>)->Result<HttpResponse, AWError>{
    zhuge::get_score()
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

    let manager = SqliteConnectionManager::file("blog.db");
    let pool = Pool::new(manager).unwrap();


    HttpServer::new(||
        App::new()
        .wrap(middleware::Logger::default())
        .service(actix_files::Files::new("/","./static/").index_file("index.html"))
    )
    .bind("172.26.242.57:80")?
    .run()
    .await
}