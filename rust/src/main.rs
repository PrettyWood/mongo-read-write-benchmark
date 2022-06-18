use futures::stream::TryStreamExt;
use mongodb::{
    bson::doc,
    options::{ClientOptions, FindOptions, ResolverConfig, DropCollectionOptions},
    Client,
};

use rand::seq::SliceRandom;

use std::{error::Error, time::Instant};

static TOTAL_DOCS: usize = 1_000_000;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client_uri = "mongodb://localhost:27017";
    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    let client = Client::with_options(options)?;

    let collection = client.database("test").collection("rust");

    collection.drop(DropCollectionOptions::builder().build()).await?;

    // INSERT DATA

    let authors = ["Eric", "Chiara", "Stew"];
    let docs = (1..=TOTAL_DOCS)
        .map(|i| {
            doc! {
                "title": format!("my doc {i}"),
                "author": authors.choose(&mut rand::thread_rng())
            }
        })
        .collect::<Vec<_>>();

    let start = Instant::now();

    collection.insert_many(docs, None).await?;

    let elapsed = start.elapsed();
    println!("Insert time for {TOTAL_DOCS} docs: {elapsed:.2?}");

    // RETRIEVE DATA

    let start = Instant::now();

    let cursor = collection
        .find(
            doc! { "author": "Chiara" },
            FindOptions::builder().sort(doc! { "title": 1_u32 }).build(),
        )
        .await?;

    let books = cursor.try_collect::<Vec<_>>().await?;
    let found_books = books.len();

    let elapsed = start.elapsed();
    println!("Read time for {TOTAL_DOCS} docs: {elapsed:.2?}");
    println!("Amount of found books: {found_books:?}");

    Ok(())
}
