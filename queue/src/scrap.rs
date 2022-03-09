use scraper::{Html, Selector};
use serde::{Serialize, Deserialize};
use sqlx::postgres::PgPool;
use anyhow::Result;
use lapin::message::Delivery;
use std::time::Duration;

use isahc::{
    config::RedirectPolicy,
    cookies::{CookieJar},
    prelude::*,
    Request,
    Response,
    Body
};

pub use crate::models::*;


#[derive(Debug, Serialize, Deserialize)]
pub struct ParseProductInfo {
    pub title: String,
    pub in_stock: bool,
}

pub async fn handle_message(delivery: &Delivery) -> Result<()> {
    let message = std::str::from_utf8(&delivery.data).unwrap();
    let merchant_product: MerchantProduct = serde_json::from_str(message).expect("Json message decoded");
    // Get url content.
    let url: String = merchant_product.url.unwrap(); 
    let mut response = call_url(&url)
        .await
        .expect("Call url");
    
    if  response.status().is_success() {
        let body = response.text()?;
        // Get scraping elements by merchant
        let scaping_elements = merchant_product.merchant.scraping_elements
            .expect("Get scraping elements");

        let title = scaping_elements.title.to_owned();
        let cart = scaping_elements.cart.to_owned();
        // Parse response body
        let parse_result = parse_body(&body, &title, &cart)
            .await
            .expect("Parsing result");
        
        println!("{:?}", parse_result);

    } else {
        // TODO manage response error
        println!("Status : {:?} - {:?}", response.status(), url);
    }
    
    Ok(())
}

async fn call_url(url: &String) -> Result<Response<Body>> {
    let cookie_jar = CookieJar::new();
    let response = Request::get(url)
        .redirect_policy(RedirectPolicy::Follow)
        .redirect_policy(RedirectPolicy::Limit(5))
        .timeout(Duration::from_secs(5))
        .cookie_jar(cookie_jar.clone())
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.0.4844.51 Safari/537.36")
        .body(())?
        .send()
        .expect("Process request");

    Ok(response)
}


async fn parse_body(body: &str, title: &String, cart: &String) -> Result<ParseProductInfo> {
    // Parsing HTML
    let document = Html::parse_document(body);
    // HTML Elements
    let title_element = Selector::parse(&title).unwrap();
    let add_to_cart_element = Selector::parse(&cart).unwrap();
    // Element values
    let has_cart_button= document.select(&add_to_cart_element).count();
    let mut title: String = String::from("");
    
    if document.select(&title_element).count() == 1 {
        let title_node= document.select(&title_element). next().unwrap();
        title = title_node.inner_html();
    }

    let scrap_product_info = ParseProductInfo {
        title: title.trim().to_string(),
        in_stock: has_cart_button == 1,
    };

    Ok(scrap_product_info)
}