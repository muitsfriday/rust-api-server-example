use actix_web::{web, App, HttpServer};
use mongodb::{options::ClientOptions, Client, Database};

mod handler;

pub async fn init_mongo(url: &str, dbname: &str) -> mongodb::error::Result<Database> {
    println!("connecting to the mongodb");

    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse(url).await?;

    // Manually set an option.
    client_options.app_name = Some("My App".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;
    let db = client.database(dbname);

    println!("connected to the mongodb");

    Ok(db)
}

pub async fn init_server(port: &str) -> std::io::Result<()> {
    let app_port = port.parse::<u16>().unwrap();

    HttpServer::new(|| {
        App::new()
            .route("/test", web::get().to(handler::test_handler))
            .route("/post", web::post().to(handler::create_post::handle))
    })
    .bind(("0.0.0.0", app_port))?
    .run()
    .await
}
