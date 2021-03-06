
use serde::{Deserialize, Serialize};
use failure::Error;
#[derive(Debug, Serialize, Deserialize)]
 pub struct Zhuge{
    pub point:u8,
}
// pub enum action{
//     add,
//     sub,
// }
pub fn get_score()->Result<Zhuge,Error>{
    let connection = sqlite::open("blog.db").unwrap();
    let mut stmt=connection.prepare("SELECT * FROM zuge_score").unwrap();
    stmt.next();
    Ok(    
        Zhuge{
        point:stmt.read::<i64>(0).unwrap() as u8,
    })
}
// pub fn change_score(action:action)->Result<(),Error>{
//     match action {
//         add=>add_score(),
//         sub=>sub_score(),
//     }
// }
pub fn add_score()->Result<(),Error>{
    let connection=sqlite::open("blog.db")?;
    let mut stme=connection.prepare("UPDATE zuge_score SET score = score+10")?;
    stme.next();
    Ok(())
}
pub fn sub_score()->Result<(),Error>{
    let connection=sqlite::open("blog.db")?;
    let mut stme=connection.prepare("UPDATE zuge_score SET score = score-10")?;
    stme.next();
    Ok(())
}