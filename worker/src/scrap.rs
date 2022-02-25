use scraper::{Html, Selector};
use sqlx::postgres::PgPool;
use futures::{stream, StreamExt}; 
use reqwest::Client;

use crate::MerchantProduct;
use crate::Tracking;

const PARALLEL_REQUESTS: usize = 6;

#[derive(Debug)]
pub struct ScrapProductInfo {
    pub title: String,
    pub in_stock: bool,
}


async fn scrap(body: &str) -> Result<ScrapProductInfo,  Box<dyn std::error::Error>> {

    // Parsing HTML
    let document = Html::parse_document(body);
    // HTML Elements
    let title_element = Selector::parse("#productTitle").unwrap();
    let add_to_cart_element = Selector::parse("#add-to-cart-button").unwrap();
    // Element values
    let has_cart_button= document.select(&add_to_cart_element).count();
    let mut title: String = String::from("");
    
    if document.select(&title_element).count() == 1 {
        let title_node= document.select(&title_element). next().unwrap();
        title = title_node.inner_html();
    }

    let scrap_product_info = ScrapProductInfo {
        title: title,
        in_stock: has_cart_button == 1,
    };

    Ok(scrap_product_info)
}

pub async fn process_request(pool: &PgPool) {
    // Get urls from Database
    let product_urls = MerchantProduct::list_tracked_products(pool).await;

    let products = match product_urls {
        Ok(p) => p,
        Err(_) => vec![],
    };

    let client = Client::new();

    let bodies = stream::iter(products)
        .map(|product| {
            let client = client.clone();
            tokio::spawn(async move {
                client.get(product.url).send().await
            })
        })
        .buffer_unordered(PARALLEL_REQUESTS);
        bodies
        .for_each(|resp| async {
            match resp {

                Ok(Ok(resp)) => {
                    // Parse good response only
                    if resp.status().is_success() {
                        let url = resp.url().as_str().to_owned();
                        let body = resp.text().await;
                        match body {
                            Ok(b) => {
                                // Scrapping response content
                                let scrap_info = scrap(&b).await;
                                match scrap_info {

                                    Ok(sp) => {
                                        // Insert results in database.
                                        Tracking::insert(&sp, &url, pool).await;
                                    },
                                    Err(e) =>  eprintln!("Got a scrapping error: {}", e),
                                }
                            },
                            Err(e) => eprintln!("Got a body response error: {}", e),
                        }
                    }
                },
                Ok(Err(e)) => eprintln!("Got a reqwest::Error: {}", e),
                Err(e) => eprintln!("Got a tokio::JoinError: {}", e),
            }
        })
        .await;
    }