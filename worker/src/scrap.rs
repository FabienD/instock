use anyhow::{anyhow, Result};
use lapin::message::Delivery;
use playwright::Playwright;
use regex::Regex;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use voca_rs::strip;

use isahc::{config::RedirectPolicy, cookies::CookieJar, prelude::*, Request};

pub use crate::models::*;

#[derive(Debug)]
pub struct CallResponse {
    pub url: String,
    pub status: u16,
    pub body: String,
}

impl CallResponse {
    fn is_success(&self) -> bool {
        self.status <= 400
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParseProductInfo {
    pub title: String,
    pub in_stock: bool,
}

pub async fn handle_message(delivery: &Delivery) -> Result<Tracking> {
    let message = std::str::from_utf8(&delivery.data).unwrap();
    let merchant_product: MerchantProduct =
        serde_json::from_str(message).expect("Json message decoded");
    // Get url content.
    let url: String = merchant_product.url.unwrap();
    
    // TODO : use call_url or call_url_browser
    let call_response = call_url(&url).await.expect("Call url");
    let tracking: Tracking;

    if call_response.is_success() {
        // Get scraping elements by merchant
        let scaping_elements = merchant_product
            .merchant
            .scraping_elements
            .expect("Get scraping elements");

        let title = scaping_elements.title.to_owned();
        let cart = scaping_elements.cart.to_owned();
        // Parse response body
        let parse_result = parse_body(&call_response.body, &title, &cart)
            .await
            .expect("Parsing result");

        tracking = Tracking {
            product_id: merchant_product.id.unwrap(),
            is_in_stock: parse_result.in_stock,
        };
        
        Ok(tracking)
    } else {
        Err(anyhow!("Handle message error"))
    }
}

async fn call_url(url: &String) -> Result<CallResponse> {
    let cookie_jar = CookieJar::new();
    let mut response = Request::get(url)
        .redirect_policy(RedirectPolicy::Follow)
        .redirect_policy(RedirectPolicy::Limit(5))
        .timeout(Duration::from_secs(5))
        .cookie_jar(cookie_jar.clone())
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.0.4844.51 Safari/537.36")
        .body(())?
        .send()
        .expect("Process request");

    let call_reponse = CallResponse {
        url: url.to_string(),
        status: response.status().as_u16(),
        body: response.text()?,
    };

    Ok(call_reponse)
}

async fn call_url_browser(url: &String) -> Result<CallResponse> {
    let playwright = Playwright::initialize().await?;
    playwright.prepare()?; // Install browsers
    let chromium = playwright.chromium();
    let browser = chromium.launcher().headless(true).launch().await?;
    let context = browser.context_builder().build().await?;
    let page = context.new_page().await?;
    let response = page.goto_builder(url).goto().await?;

    let mut status: i32 = 999;
    let mut body: String = "".to_string();
    
    match response {
        Some(response) => {
            status = response.status().expect("Response status");
            body = response.text().await.expect("Response content");
        },
        None => println!("{} call failed!", url),
    }

    let call_reponse = CallResponse {
        url: url.to_string(),
        status: status as u16,
        body,
    };

    Ok(call_reponse)
}

async fn parse_body(body: &str, title: &String, cart: &String) -> Result<ParseProductInfo> {
    // Parsing HTML
    let document = Html::parse_document(body);
    // HTML Elements
    let title_element = Selector::parse(&title).unwrap();
    let add_to_cart_element = Selector::parse(&cart).unwrap();
    // Element values
    let has_cart_button = document.select(&add_to_cart_element).count();
    let mut title: String = String::from("");

    if document.select(&title_element).count() == 1 {
        let title_node = document.select(&title_element).next().unwrap();
        title = title_node.inner_html().to_string();
        title = clean_title(&title).await.expect("Clean title");
    }

    let scrap_product_info = ParseProductInfo {
        title: title,
        in_stock: has_cart_button == 1,
    };

    Ok(scrap_product_info)
}

async fn clean_title(title: &String) -> Result<String> {
    let mut title = strip::strip_tags(title.trim());
    let re = Regex::new(r"[\t|\n|\r]+").unwrap();
    title = re.replace_all(title.as_str(), " ").to_string();

    Ok(title)
}
