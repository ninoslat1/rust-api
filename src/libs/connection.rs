use dotenv::dotenv;
use std::env;

use sqlx::{ Error, MySqlPool};

pub async fn connect_user() -> Result<MySqlPool, Error> {
    dotenv().ok(); // Load .env variables
    let database_url = env::var("DATABASE_USER_URL")
        .expect("DATABASE_USER_URL must be set");
    MySqlPool::connect(&database_url).await
}


// pub async fn connect_master() -> Result<Pool<MySql>, Error> {
//     return MySqlPool::connect("mysql://root:cog938gb18@localhost:3306/bromomst").await;
// }