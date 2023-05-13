use mysql::prelude::*;
use mysql::Pool;

pub fn insert(points: i32) -> Result<(), mysql::Error> {
    // Connect to the MySQL database
    let url = "mysql://root:root@localhost:3306/pesmario";
    let pool = Pool::new(url)?;

    // Insert the score into the Leaderboard table
    let mut conn = pool.get_conn()?;
    let query = format!("INSERT INTO Leaderboard VALUES ('{}', {})",
                         LoginPanel::get_username(), points);
    conn.exec_drop(query)?;

    Ok(())
}

