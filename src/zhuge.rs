use serde::{Deserialize, Serialize};
use actix_web::{web,Error as AWError};
use failure::Error;
use std::{thread::sleep, time::Duration};
use futures::{Future,TryFutureExt};
use rusqlite::{Statement,NO_PARAMS};
use std::iter::Iterator;
pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;
type ZugeResult = Result<Vec<Zhuge>,rusqlite::Error>;
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
impl Future<Output=Result<usize,AWError>>{
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
fn get_score(pool:&Pool) ->ZugeResult {
    let conn=pool.get().unwrap();
    let mut  stmt=conn.prepare(
        "
        SELECT * FROM zuge_score
        ",
    )?;
    stmt.query_map(NO_PARAMS,|row| {
        Ok(
            Zhuge{
                point:row.get(0)?,
            }
        )
    }).and_then(Iterator::collect)
}
fn get_zuge_score(mut statement:Statement)->ZugeResult{
    statement.query_map(NO_PARAMS,|row| {
        Ok(
            Zhuge{
                point:row.get(0)?,
            }
        )
    }).and_then(Iterator::collect)
}
fn add_score(conn:Connection)->Result<usize,rusqlite::Error>{
    conn.execute(
        "UPDATE zuge_score score=score+10",
        NO_PARAMS,
    )
}
fn sub_score(conn:Connection)->Result<usize,rusqlite::Error> {
    conn.execute(
        "UPDATE zuge_score score=score-10",
        NO_PARAMS,
    )
}


