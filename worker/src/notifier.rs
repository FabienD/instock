use anyhow::Result;
use lapin::message::Delivery;
use lettre::{message::MultiPart, AsyncSmtpTransport, AsyncStd1Executor, AsyncTransport, Message};
use sqlx::PgPool;
use voca_rs::strip;

pub use crate::models::*;

pub async fn handle_message(
    delivery: &Delivery,
    pool: &PgPool,
    mailer: &AsyncSmtpTransport<AsyncStd1Executor>,
    notification_from: &String,
) {
    let message = std::str::from_utf8(&delivery.data).unwrap();
    let product: TrackedProduct = serde_json::from_str(&message).unwrap();

    // Get users who subscribes to each product.
    let subscribers = get_user_by_product_id(&product, &pool).await.unwrap();

    // Send notification using SMTP
    for subscriber in subscribers {
        let is_sent = send_notification(&subscriber, &product, &mailer, &notification_from)
            .await
            .unwrap();

        if is_sent {
            update_user_tracking(&subscriber, &product, &pool)
                .await
                .unwrap();
        }
    }
}

async fn get_user_by_product_id(
    product: &TrackedProduct,
    pool: &PgPool,
) -> Result<Vec<Subscriber>> {
    let subscribers = sqlx::query_as!(
        Subscriber,
        r#"
        SELECT
            us.id,
            us.email
        FROM instock.user AS us
        JOIN instock.user_tracking AS ut ON ut.user_id = us.id
        WHERE us.is_enabled
          AND ut.product_id = $1
          AND ut.alert_count < ut.alert_count_max
        "#,
        &product.product_id
    )
    .fetch_all(pool)
    .await?;

    Ok(subscribers)
}

async fn send_notification(
    subscriber: &Subscriber,
    product: &TrackedProduct,
    mailer: &AsyncSmtpTransport<AsyncStd1Executor>,
    notification_from: &String,
) -> Result<bool> {
    let product_name = &product.product_name;
    let product_links = build_product_links(&product.links);

    // Email Headers
    let from = format!("Instock <{notification_from}>");
    let to = &subscriber.email;
    let subject = format!("Intock - {product_name} is back");
    // Email content
    let body = format!(
        r#"
    <div>
        <p>
            Hello,
            <br /><br />
            <strong>{product_name}</strong> is back in stock :
            <ul>
                {product_links}
            </ul>
            <br />
            Enjoy!
        </p>
    </div>
    "#
    );

    let email = Message::builder()
        .from(from.parse()?)
        .to(to.parse()?)
        .subject(subject)
        .multipart(MultiPart::alternative_plain_html(
            String::from(strip::strip_tags(&body)),
            String::from(&body),
        ))?;

    let sending = mailer.send(email).await?;

    Ok(sending.is_positive())
}

fn build_product_links(links: &Vec<TrackedProductLinks>) -> String {
    let mut template: String = "".to_string();

    for link in links {
        let merchant_product_url = &link.merchant_product_url;
        let merchant = &link.merchant;
        let price = &link.price;
        let date = &link.tracked_at.format("%Y-%m-%d %H:%M").to_string();

        template = format!("{template} <li><a href=\"{merchant_product_url}\">{price} sur {merchant} <i>({date})</i></a></li>");
    }

    return template;
}

async fn update_user_tracking(
    subscriber: &Subscriber,
    product: &TrackedProduct,
    pool: &PgPool,
) -> Result<()> {
    sqlx::query!(
        r#"
            UPDATE instock.user_tracking
            SET alert_count = alert_count + 1
            WHERE user_id = $1
              AND product_id = $2
        "#,
        &subscriber.id,
        &product.product_id
    )
    .execute(pool)
    .await?;

    Ok(())
}
