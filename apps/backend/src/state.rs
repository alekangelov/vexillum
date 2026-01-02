use crate::config::Config;
use deadpool_postgres::{self, ManagerConfig, RecyclingMethod};
use std::sync::Arc;
use tokio_postgres::NoTls;

#[derive(Clone)]
pub struct BaseState {
    pub db_pool: deadpool_postgres::Pool,
    pub redis_pool: deadpool_redis::Pool,
    pub config: Arc<Config>,
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
            redis_pool: redis_pool,
        };

        Ok(base_state)
    }

    pub async fn new_arc(config: Config) -> Result<AppState, Box<dyn std::error::Error>> {
        let state = Self::new(config).await?;
        Ok(Arc::new(state))
    }
}
