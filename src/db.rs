use mongodb::{ 
	bson::{Document, doc},
	Client,
	Collection 
};

pub async fn test_connection() -> mongodb::error::Result<()> {
    // Replace the placeholder with your Atlas connection string
    let conn_string = std::env::var("DATABASE_URL").expect("database URL must be set.");
    // Create a new client and connect to the server
    let client = Client::with_uri_str(conn_string).await?;

    // Get a handle on the movies collection
    let database = client.database("sample_mflix");
    let my_coll: Collection<Document> = database.collection("movies");

    // Find a movie based on the title value
    let my_movie = my_coll.find_one(doc! { "title": "The Perils of Pauline" }, None).await?;

    // Print the document
    println!("Found a movie:\n{:#?}", my_movie);
    Ok(())
}

pub async fn db(collection: &str) -> mongodb::error::Result<()> {
    // Replace the placeholder with your Atlas connection string
    let conn_string = std::env::var("DATABASE_URL").expect("database URL must be set.");

    // Create a new client and connect to the server
    let client = Client::with_uri_str(conn_string).await?;

    // Get a handle on the movies collection
    let database = client.database("sample_mflix");
    let my_coll: Collection<Document> = database.collection(collection);

    // Find a movie based on the title value
    let my_movie = my_coll.find_one(doc! { "title": "The Perils of Pauline" }, None).await?;

    // Print the document
    println!("Found a movie:\n{:#?}", my_movie);
    Ok(())
}