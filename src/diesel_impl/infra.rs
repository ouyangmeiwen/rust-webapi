use diesel;
use diesel::r2d2::ConnectionManager;
use std::env;
use dotenv::dotenv; // 引入 dotenv

pub type Pool<T> = r2d2::Pool<ConnectionManager<T>>;
// PostgreSQL connection pool type
pub type PgPool = Pool<diesel::pg::PgConnection>;

// Define DBConn type as PostgreSQL pool
#[cfg(feature = "postgres")]
pub type DBConn = PgPool;

pub fn db_pool() -> DBConn {
    // Load environment variables from .env file
    dotenv().ok();  // 加载 .env 文件

    // Get the DATABASE_URL from environment variables
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        // Default to PostgreSQL if DATABASE_URL is not set
        "postgres://username:password@localhost/dbname".to_string()
    });

    println!("Using Database: {}", database_url);

    // Create PostgreSQL connection manager
    let manager = ConnectionManager::<diesel::pg::PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create PostgreSQL pool")
}
