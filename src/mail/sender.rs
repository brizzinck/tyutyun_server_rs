use crate::error::api_error::ApiError;
use lettre::message::{Message, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::SmtpTransport;
use lettre::Transport;
use std::env;
use crate::data::orders::order::OrderDetails;

pub fn generate_registration_link(token: String) -> String {
    format!(
        "http://{}:{}/api/registration?token={}",
        env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1".to_string()),
        env::var("SERVER_PORT").unwrap_or("8181".to_string()),
        token
    )
}

pub fn send_mail_registration(to_email: String, active_link: String) -> Result<String, ApiError> {
    let smtp_server = env::var("SMTP_SERVER").map_err(|_| ApiError::EmailError)?;
    let smtp_port: u16 = env::var("SMTP_PORT")
        .unwrap_or("587".to_string())
        .parse()
        .map_err(|_| ApiError::EmailError)?;
    let username = env::var("MAIL_USERNAME").map_err(|_| ApiError::EmailError)?;
    let password = env::var("MAIL_PASSWORD").map_err(|_| ApiError::EmailError)?;

    let html_content = format!(
        r#"
            <!DOCTYPE html>
            <html>
            <head>
                <style>
                    body {{
                        background-color: #1a1a1a;
                        color: white;
                        font-family: Namu, sans-serif;
                        margin: 0;
                        padding: 20px;
                    }}
                    .button {{
                        display: inline-block;
                        padding: 10px 20px;
                        font-size: 16px;
                        color: #000000;
                        background-color: #FFA500;
                        border: none;
                        border-radius: 5px;
                        text-decoration: none;
                        cursor: pointer;
                    }}
                    .button:hover {{
                        background-color: #e59400;
                    }}
                    .text {{
                        color: #FFA500;
                    }}
                    .container {{
                        max-width: 600px;
                        margin: 0 auto;
                        padding: 20px;
                        background-color: #1a1a1a;
                        border-radius: 10px;
                    }}
                </style>
            </head>
            <body>
                <div class="container">
                    <h2 class="text">Хелоу це Tyuntyun Shop!</h2>
                    <p class="text">Будь ласка активуй свій аккаунт, натисни кнопку нижче:</p>
                    <a href="{link}" class="button">Активувати аккаунт</a>
                    <p class="text">Дякуууую що ти з нами!</p>
                </div>
            </body>
            </html>
        "#,
        link = active_link
    );

    let email = Message::builder()
        .from(
            "Tyutyun Shop <tyutyun-shop@yacode.dev>"
                .parse()
                .map_err(|_| ApiError::EmailError)?,
        )
        .to(to_email.parse().map_err(|_| ApiError::EmailError)?)
        .subject("Активація аккаунта - Tyutyun Shop")
        .singlepart(SinglePart::html(html_content))
        .map_err(|_| ApiError::EmailError)?;

    let creds = Credentials::new(username.clone(), password.clone());
    let mailer = SmtpTransport::starttls_relay(&smtp_server)
        .map_err(|_| ApiError::EmailError)?
        .port(smtp_port)
        .credentials(creds)
        .build();

    mailer.send(&email).map_err(|_| ApiError::EmailError)?;

    Ok(format!(
        "Activation email sent successfully to {}",
        to_email
    ))
}


pub fn send_mail_new_order(order_details: OrderDetails) -> Result<String, ApiError> {
    let smtp_server = env::var("SMTP_SERVER").map_err(|_| ApiError::EmailError)?;
    let smtp_port: u16 = env::var("SMTP_PORT")
        .unwrap_or("587".to_string())
        .parse()
        .map_err(|_| ApiError::EmailError)?;
    let username = env::var("MAIL_USERNAME").map_err(|_| ApiError::EmailError)?;
    let password = env::var("MAIL_PASSWORD").map_err(|_| ApiError::EmailError)?;

    let items_html = order_details.items.iter().map(|item| {
        format!(
            r#"<tr>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{} грн</td>
            </tr>"#,
            item.product_name,
            item.quantity,
            item.size.clone().unwrap_or_else(|| "N/A".to_string()),
            item.total_price
        )
    }).collect::<String>();

    let html_content = format!(
        r#"
    <!DOCTYPE html>
    <html>
    <head>
        <style>
            body {{
                background-color: #1c1c1c;
                color: #FFA500;
                font-family: 'Namu', sans-serif;
                margin: 0;
                padding: 20px;
            }}
            .container {{
                max-width: 600px;
                margin: auto;
                padding: 20px;
                background-color: #1c1c1c;
                border-radius: 8px;
                border: 1px solid #FFA500;
            }}
            .header {{
                color: #FFA500;
                font-size: 24px;
                text-align: center;
                margin-bottom: 20px;
                font-weight: bold;
            }}
            .details {{
                margin-top: 20px;
            }}
            .details p {{
                margin: 5px 0;
                color: #e59400;
            }}
            table {{
                width: 100%;
                border-collapse: collapse;
                margin-top: 10px;
                background-color: #1111;
            }}
            th, td {{
                border: 1px solid #FFA500;
                padding: 12px;
                text-align: center;
                color: #e59400;
            }}
            th {{
                background-color: #FFA500;
                color: #000;
            }}
            tr:nth-child(even) {{
                background-color: #1c1c1c;
            }}
            tr:nth-child(odd) {{
                background-color: #292929;
            }}
            .button {{
                display: inline-block;
                margin-top: 20px;
                padding: 10px 20px;
                background-color: #FFA500;
                color: #000;
                text-decoration: none;
                border-radius: 4px;
                text-align: center;
                font-weight: bold;
                cursor: pointer;
            }}
            .button:hover {{
                background-color: #e59400;
                color: #1a1a1a;
            }}
        </style>
    </head>
    <body>
        <div class="container">
            <h2 class="header">Деталі нового замовлення</h2>
            <div class="details">
                <h3 style="color: #FFA500;">Інформація про доставку</h3>
                <p><strong>Адреса:</strong> {address}</p>
                <p><strong>Ім'я:</strong> {first_name} {last_name}</p>
                <p><strong>Телефон:</strong> {phone}</p>
                <p><strong>Пошта:</strong> {email}</p>
            </div>
            <div class="details">
                <h3 style="color: #FFA500;">Товари</h3>
                <table>
                    <thead>
                        <tr>
                            <th>Назва</th>
                            <th>Кількість</th>
                            <th>Розмір</th>
                            <th>Ціна</th>
                        </tr>
                    </thead>
                    <tbody>
                        {items}
                    </tbody>
                </table>
                <p style="text-align: right; font-size: 18px; margin-top: 20px; color: #FFA500;">
                    <strong>Загальна сума:</strong> {total_price} грн
                </p>
            </div>
        </div>
    </body>
    </html>
    "#,
        first_name = order_details.shipping.first_name,
        last_name = order_details.shipping.last_name,
        address = order_details.shipping.address,
        phone = order_details.shipping.phone_number,
        email = order_details.shipping.email,
        items = items_html,
        total_price = order_details.items.iter().map(|item| item.total_price).sum::<f32>(),
    );


    let email = Message::builder()
        .from(
            "Tyutyun Shop <tyutyun-shop@yacode.dev>"
                .parse()
                .map_err(|_| ApiError::EmailError)?,
        )
        .to(order_details.shipping.email.parse().map_err(|_| ApiError::EmailError)?)
        .subject("Деталі нового замовлення - Tyutyun Shop")
        .singlepart(SinglePart::html(html_content))
        .map_err(|_| ApiError::EmailError)?;

    let creds = Credentials::new(username.clone(), password.clone());
    let mailer = SmtpTransport::starttls_relay(&smtp_server)
        .map_err(|_| ApiError::EmailError)?
        .port(smtp_port)
        .credentials(creds)
        .build();

    mailer.send(&email).map_err(|_| ApiError::EmailError)?;

    Ok(format!(
        "Order confirmation email sent successfully to {}",
        order_details.shipping.email
    ))
}

