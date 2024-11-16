use dotenvy::dotenv;
use eyre::Result;
use sqlx::{postgres::PgPoolOptions, Connection, Executor, PgConnection, PgPool};
use std::env;

pub async fn init_db_pool() -> Result<PgPool> {
    dotenv().ok();

    let main_database_url =
        env::var("MAIN_DATABASE_URL").expect("MAIN_DATABASE_URL must be set in the .env file");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in the .env file");
    let db_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set in the .env file");

    let mut main_conn = PgConnection::connect(&main_database_url).await?;

    let db_check_query = format!("SELECT 1 FROM pg_database WHERE datname = '{}';", db_name);
    let db_exists: Option<(i32,)> = sqlx::query_as(&db_check_query)
        .fetch_optional(&mut main_conn)
        .await?;

    if db_exists.is_none() {
        sqlx::query(&format!("CREATE DATABASE {};", db_name))
            .execute(&mut main_conn)
            .await
            .expect("Failed to create database");
    }

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    pool.execute(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            username VARCHAR(255) NOT NULL UNIQUE,
            email VARCHAR(255) NOT NULL UNIQUE,
            password_hash VARCHAR(255) NOT NULL,
            first_name VARCHAR(100),
            last_name VARCHAR(100),
            address VARCHAR(100),
            phone_number VARCHAR(20) UNIQUE,
            role VARCHAR(20) NOT NULL DEFAULT USER,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS categories (
            id SERIAL PRIMARY KEY,
            name VARCHAR(255) NOT NULL UNIQUE,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS product_images (
            id SERIAL PRIMARY KEY,
            product_id INT,
            image_url VARCHAR(255) NOT NULL,
            alt_text VARCHAR(255),
            position INT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS product_sizes (
            id SERIAL PRIMARY KEY,
            product_id INT,
            single_size INT DEFAULT 0,
            S INT DEFAULT 0,
            M INT DEFAULT 0,
            L INT DEFAULT 0,
            XL INT DEFAULT 0,
            XXL INT DEFAULT 0,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS products (
            id SERIAL PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            description TEXT,
            primary_image_id INT REFERENCES product_images(id) ON DELETE SET NULL,
            price REAL NOT NULL,
            category_id INT REFERENCES categories(id) ON DELETE SET NULL,
            size_id INT REFERENCES product_sizes(id) ON DELETE SET NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS orders (
            id SERIAL PRIMARY KEY,
            user_id INT REFERENCES users(id) ON DELETE SET NULL DEFAULT NULL,
            total_price REAL NOT NULL,
            status VARCHAR(50) DEFAULT 'pending',
            online_payment BOOLEAN NOT NULL DEFAULT FALSE,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS shipping_addresses (
            id SERIAL PRIMARY KEY,
            order_id INT REFERENCES orders(id) ON DELETE SET NULL,
            address VARCHAR(255) NOT NULL,
            first_name VARCHAR(100),
            last_name VARCHAR(100),
            phone_number VARCHAR(20),
            email VARCHAR(255),
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS order_items (
            id SERIAL PRIMARY KEY,
            order_id INT REFERENCES orders(id) ON DELETE SET NULL,
            product_id INT REFERENCES products(id),
            quantity INT NOT NULL,
            price REAL NOT NULL,
            size VARCHAR(25) DEFAULT NULL,
            total_price REAL GENERATED ALWAYS AS (quantity * price) STORED
        );

        CREATE TABLE IF NOT EXISTS payments (
            id SERIAL PRIMARY KEY,
            order_id INT REFERENCES orders(id) ON DELETE SET NULL,
            payment_method VARCHAR(50) NOT NULL,
            payment_status VARCHAR(50) DEFAULT 'pending',
            amount REAL NOT NULL,
            payment_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
        "#,
    )
    .await?;

    Ok(pool)
}
