use clap::Parser;
use dotenvy::dotenv;

#[derive(Parser, Debug, Clone)]
#[command(name = "Vexillum Backend")]
#[command(about = "A backend service built with Axum and Tokio Postgres")]
pub struct Config {
    /// PostgreSQL host
    #[arg(long, env = "DATABASE_HOST", default_value = "localhost")]
    pub database_host: String,

    /// PostgreSQL port
    #[arg(long, env = "DATABASE_PORT", default_value = "5432")]
    pub database_port: u16,

    /// PostgreSQL user
    #[arg(long, env = "DATABASE_USER", default_value = "postgres")]
    pub database_user: String,

    /// PostgreSQL password
    #[arg(long, env = "DATABASE_PASSWORD", default_value = "postgres")]
    pub database_password: String,

    /// PostgreSQL database name
    #[arg(long, env = "DATABASE_NAME", default_value = "vexillum")]
    pub database_name: String,

    /// Server host
    #[arg(long, env = "SERVER_HOST", default_value = "127.0.0.1")]
    pub server_host: String,

    /// Server port
    #[arg(long, env = "SERVER_PORT", default_value = "3000")]
    pub server_port: u16,

    /// Log level
    #[arg(long, env = "LOG_LEVEL", default_value = "info")]
    pub log_level: String,

    /// Redis host
    #[arg(long, env = "REDIS_HOST", default_value = "127.0.0.1")]
    pub redis_host: String,

    /// Redis port
    #[arg(long, env = "REDIS_PORT", default_value = "6379")]
    pub redis_port: u16,

    /// Admin user email
    #[arg(long, env = "ADMIN_EMAIL", default_value = "")]
    pub admin_email: String,

    /// Admin user password
    #[arg(long, env = "ADMIN_PASSWORD", default_value = "")]
    pub admin_password: String,
}

impl Config {
    /// Load configuration from .env file and environment variables
    pub fn load() -> Self {
        // Load .env file if it exists
        let _ = dotenv();

        // Parse arguments with environment variable fallback
        

        Config::parse()
    }

    /// Build the PostgreSQL connection string
    pub fn database_url(&self) -> String {
        format!(
            "host={} port={} user={} password={} dbname={}",
            self.database_host,
            self.database_port,
            self.database_user,
            self.database_password,
            self.database_name
        )
    }

    /// Get the server address
    pub fn server_addr(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }

    /// Print configuration (without sensitive data)
    pub fn print_summary(&self) {
        println!("Configuration loaded:");
        println!(
            "  Database: {}@{}:{}/{}",
            self.database_user, self.database_host, self.database_port, self.database_name
        );
        println!("  Server: http://{}", self.server_addr());
        println!("  Log Level: {}", self.log_level);
    }

    pub fn redis_addr(&self) -> String {
        format!("redis://{}:{}", self.redis_host, self.redis_port)
    }
}
