use scraper::{Html, Selector};
use sqlx::postgres::PgPool;
use futures::{stream, StreamExt}; 
use reqwest::Client;
use reqwest::StatusCode;

use crate::ProductUrl;

const PARALLEL_REQUESTS: usize = 6;

async fn scrap(body: &str) {
    // Parsing HTML
    let document = Html::parse_document(body);
    // HTML Elements
    
    let title_element = Selector::parse("#productTitle").unwrap();
    let add_to_cart_element = Selector::parse("#add-to-cart-button").unwrap();
    // Element values
    let has_cart_button= document.select(&add_to_cart_element).count();
    
    if document.select(&title_element).count() == 1 {
        let title= document.select(&title_element). next().unwrap();
    
        if has_cart_button == 1 {
            println!("{} in stock", title.inner_html());
        } else {
            println!("{} Out of stock", title.inner_html());
        }
    }
}

pub async fn process_request(pool: &PgPool) {
    // Get urls from Database
    let product_urls = ProductUrl::list_tracked_products(pool).await;

    let products = match product_urls {
        Ok(p) => p,
        Err(_) => vec![],
    };

    let client = Client::new();

    let bodies = stream::iter(products)
        .map(|product| {
            let client = client.clone();
            tokio::spawn(async move {
                let resp = client.get(product.url).send().await?;
                resp.text().await
            })
        })
        .buffer_unordered(PARALLEL_REQUESTS);
        bodies
        .for_each(|b| async {
            match b {
                Ok(Ok(b)) => scrap(&b).await,
                Ok(Err(e)) => eprintln!("Got a reqwest::Error: {}", e),
                Err(e) => eprintln!("Got a tokio::JoinError: {}", e),
            }
        })
        .await;
    }