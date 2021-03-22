use mongodb::{bson::doc, options::ClientOptions, Client};
use chrono::prelude::*;

pub fn hello() {
    println!("hello")
}


pub async fn poplate_db() -> mongodb::error::Result<()> {
    // Parse your connection string into an options struct
    let mut client_options =
        ClientOptions::parse("mongodb://127.0.2.15:27017/test?w=majority")
            .await?;

    // Manually set an option
    client_options.app_name = Some("Rust Demo".to_string());

    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;

    // Ping the server to see if you can connect to the cluster
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("Connected successfully.");

    // List the names of the databases in that cluster
    let nestbox = client.database("nestbox");
    let breeds = nestbox.collection("breeds");
    for _i in  0..10000 {
        let _result = breeds.insert_one(doc!{"name": format!("breed_eleven {}", _i), "date": Utc::now()}, None).await?;
        //println!("{:#?}", result.inserted_id.as_object_id().unwrap());
    } 
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }
    Ok(())
}