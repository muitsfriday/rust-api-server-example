use mongodb::{options::ClientOptions, Client, Database};

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
