use std::env;

use rust_api_server_example::{self, init_mongo, init_server};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let mongo_url = env::var("MONGO_URL").expect("MONGO_URL is not found in env");
    let mongo_dbname = env::var("MONGO_DBNAME").expect("MONGO_DBNAME is not found in env");
    let _ = init_mongo(&mongo_url, &mongo_dbname)
        .await
        .expect("unable to connect to mongodb");

    let port = env::var("APP_PORT").expect("APP_PORT is not found in env");
    
    init_server(&port).await
}
