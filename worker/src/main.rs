use scraper::{Html, Selector};
use futures::{stream, StreamExt}; 
use reqwest::Client; 

const PARALLEL_REQUESTS: usize = 4;

async fn scrap(body: &str) {
    // Parsing HTML
    let document = Html::parse_document(body);
    // HTML Elements
    let title_element = Selector::parse("#productTitle").unwrap();
    let add_to_cart_element = Selector::parse("#add-to-cart-button").unwrap();
    // Element values
    let has_cart_button= document.select(&add_to_cart_element).count();
    let title= document.select(&title_element).next().unwrap();

    if has_cart_button == 1 {
        println!("{} in stock", title.inner_html());
    } else {
        println!("{} Out of stock", title.inner_html());
    }
}


async fn process_request() {

    // Product URL
    let urls = vec![
        "https://www.amazon.fr/PlayStation-%C3%89dition-Standard-DualSense-Couleur/dp/B08H93ZRK9",
        "https://www.amazon.fr/dp/B08XM9C8P6?ref=MarsFS_SMP_rvp",
        "https://www.amazon.fr/PlayStation-Digital-Manette-DualSense-Couleur/dp/B08H98GVK8"
    ];

    let client = Client::new();
    
    let bodies = stream::iter(urls)
        .map(|url| {
            let client = client.clone();
            tokio::spawn(async move {
                let resp = client.get(url).send().await?;
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


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // Get then parse all urls results
    process_request().await;
        
    Ok(())
}
