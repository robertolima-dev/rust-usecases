use mongodb::{Client, options::ClientOptions};

pub async fn init_mongodb() -> mongodb::error::Result<mongodb::Database> {
    let uri = std::env::var("MONGO_URI").expect("MONGO_URI não definida no .env");

    let options = ClientOptions::parse(uri).await?;
    let client = Client::with_options(options)?;
    Ok(client.database("nome_do_banco"))
}
