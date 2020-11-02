use serde::{Deserialize, Serialize};
use actix_web::{web,Error as AWError};
use failure::Error;
use std::{thread::sleep, time::Duration};
use futures::{Future,TryFutureExt};
pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;
type ZugeResult = Result<Zhuge,rusqlite::Error>;
//猪哥类
#[derive(Debug, Serialize, Deserialize)]
pub struct Zhuge{
    point:u8,
}
pub enum action{
    add,
    sub,
}
pub fn execute(
    pool:&Pool,
    action:action,
)->
impl Future<Output=Result<Zhuge,AWError>>{
    let pool=pool.clone();
    web::block(move||{
        sleep(Duration::from_secs(2));
        let result=match action {
            action::add=>add_score(pool.get()?),
            action::sub=>sub_score(pool.get()?),    
        };
        result.map_err(Error::from)
    })
    .map_err(AWError::from)
}
fn add_score(conn:Connection) ->ZugeResult {
    let stmt=conn.prepare(
        "
        SELECT * FROM zuge_score
        ",
    )?;
    
}


