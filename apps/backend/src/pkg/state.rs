use super::config::Config;
use crate::models::enums::UserRole;
use argon2::password_hash::{PasswordHasher, SaltString};
use deadpool_postgres::{self, ManagerConfig, RecyclingMethod};
use rand::TryRngCore;
use std::sync::Arc;
use tokio_postgres::NoTls;

#[derive(Clone)]
pub struct BaseState {
    pub db_pool: deadpool_postgres::Pool,
    pub redis_pool: deadpool_redis::Pool,
    pub config: Arc<Config>,
    pub argon2: argon2::Argon2<'static>,
}

pub type AppState = Arc<BaseState>;

impl BaseState {
    pub async fn new(config: Config) -> Result<Self, Box<dyn std::error::Error>> {
        // Connect to PostgreSQL
        let pg_config = deadpool_postgres::Config {
            user: Some(config.database_user.clone()),
            password: Some(config.database_password.clone()),
            host: Some(config.database_host.clone()),
            port: Some(config.database_port),
            dbname: Some(config.database_name.clone()),
            pool: None,
            manager: Some(ManagerConfig {
                recycling_method: RecyclingMethod::Fast,
            }),
            ..Default::default()
        };

        let pg_pool = pg_config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;

        let redis_config = deadpool_redis::Config {
            url: Some(config.redis_addr()),
            pool: None,
            connection: None,
        };
        let redis_pool = redis_config.create_pool(Some(deadpool_redis::Runtime::Tokio1));

        redis_pool
            .as_ref()
            .map_err(|e| format!("Failed to create Redis pool: {}", e))?;

        let redis_pool = redis_pool.unwrap();

        let base_state = BaseState {
            db_pool: pg_pool,
            config: Arc::new(config),
            redis_pool,
            argon2: argon2::Argon2::default(),
        };

        base_state.init_admin_user().await?;

        Ok(base_state)
    }

    pub async fn new_arc(config: Config) -> Result<AppState, Box<dyn std::error::Error>> {
        let state = Self::new(config).await?;
        Ok(Arc::new(state))
    }

    pub async fn init_admin_user(&self) -> Result<(), Box<dyn std::error::Error>> {
        let admin_email = self.config.admin_email.clone();
        let admin_password = self.config.admin_password.clone();

        let client = self.db_pool.get().await?;

        let row = client
            .query_opt("SELECT id FROM users WHERE email = $1", &[&admin_email])
            .await?;

        if row.is_none() {
            let salt = rand::rngs::OsRng
                .try_next_u64()
                .map_err(|e| format!("Failed to generate salt: {}", e))
                .and_then(|num| {
                    SaltString::encode_b64(&num.to_le_bytes())
                        .map_err(|e| format!("Failed to encode salt: {}", e))
                })?;

            let hashed_password = self
                .argon2
                .hash_password(admin_password.as_bytes(), &salt)
                .map_err(|e| format!("Failed to hash password: {}", e))?
                .to_string();

            client
                .execute(
                    "INSERT INTO users (email, password_hash, role) VALUES ($1, $2, $3)",
                    &[&admin_email, &hashed_password, &UserRole::Admin],
                )
                .await?;

            println!("Admin user created with email: {}", admin_email);
        } else {
            println!("Admin user already exists with email: {}", admin_email);
        }

        Ok(())
    }
}
