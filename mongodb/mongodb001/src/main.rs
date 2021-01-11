use mongodb::{error::Result, options::{ClientOptions, FindOptions}, Client};
use bson::{doc, bson, Bson};

fn main() {
    let client = connect_mongo().expect("failed to connect");
    list_db(&client).expect("failed to list db names");
    list_collection(&client, "test").expect("failed to list collection names");
    insert_docs(&client).expect("failed to insert docs");
    find_docs(&client).expect("failed to find docs");
}

fn connect_mongo() -> Result<Client> {
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017")?;

    // Manually set an option.
    client_options.app_name = Some("My App".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;
    Ok(client)
}

fn list_db(client: &Client) -> Result<()> {
    // List the names of the databases in that deployment.
    println!("------ print db --------");
    for db_name in client.list_database_names(None)? {
        println!("{}", db_name);
    }
    Ok(())
}

fn list_collection(client: &Client, dbname: &str) -> Result<()> {
    // Get a handle to a database.
    let db = client.database(dbname);

    // List the names of the collections in that database.
    println!("------ print collection ({}) --------", dbname);
    for collection_name in db.list_collection_names(None)? {
        println!("{}", collection_name);
    }
    Ok(())
}

fn insert_docs(client: &Client) -> Result<()> {
    // Get a handle to a collection in the database.
    let collection = client.database("test").collection("books");

    let docs = vec![
        doc! { "title": "1984", "author": "George Orwell" },
        doc! { "title": "Animal Farm", "author": "George Orwell" },
        doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
    ];

    // Insert some documents into the "test.books" collection.
    collection.insert_many(docs, None)?;

    Ok(())
}

fn find_docs(client: &Client) -> Result<()> {
    // Get a handle to a collection in the database.
    let collection = client.database("test").collection("books");
    // Query the documents in the collection with a filter and an option.
    let filter = doc! { "author": "George Orwell" };
    let find_options = FindOptions::builder().sort(doc! { "title": 1 }).build();
    let cursor = collection.find(filter, find_options)?;

    println!("------ print find  --------");
    // Iterate over the results of the cursor.
    for result in cursor {
        match result {
            Ok(document) => {
                if let Some(title) = document.get("title").and_then(Bson::as_str) {
                    println!("title: {}", title);
                }  else {
                    println!("no title found");
                }
            }
            Err(e) => return Err(e.into()),
        }
    }

    Ok(())
}