use chrono::Local;
use clap::{Parser, Subcommand};
use dotenvy::dotenv;
use std::fs;
use std::path::PathBuf;
use tokio_postgres::NoTls;

#[derive(Parser)]
#[command(name = "migrate")]
#[command(about = "Database migration tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(long, env = "DATABASE_URL")]
    database_url: String,

    #[arg(long, default_value = "migrations")]
    migrations_dir: PathBuf,
}

#[derive(Subcommand)]
enum Commands {
    /// Run pending migrations
    Up,
    /// Rollback the last migration
    Down,
    /// Create a new migration file
    Create {
        /// Name of the migration
        name: String,
    },
    /// Rollback all migrations
    Reset,
    /// Show migration status
    Status,
}

struct Migrator {
    migrations_dir: PathBuf,
    db_client: tokio_postgres::Client,
}

impl Migrator {
    async fn new(
        db_client: tokio_postgres::Client,
        migrations_dir: impl AsRef<std::path::Path>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let migrations_dir = migrations_dir.as_ref().to_path_buf();

        // Create migrations directory if it doesn't exist
        fs::create_dir_all(&migrations_dir)?;

        Ok(Migrator {
            migrations_dir,
            db_client,
        })
    }

    /// Initialize the migrations table if it doesn't exist
    async fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.db_client
            .execute(
                "CREATE TABLE IF NOT EXISTS _schema_migrations (
                    id SERIAL PRIMARY KEY,
                    name VARCHAR(255) UNIQUE NOT NULL,
                    executed_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
                )",
                &[],
            )
            .await?;
        Ok(())
    }

    /// Get all executed migrations from database
    async fn get_executed_migrations(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let rows = self
            .db_client
            .query(
                "SELECT name FROM _schema_migrations ORDER BY executed_at",
                &[],
            )
            .await?;

        Ok(rows.iter().map(|row| row.get(0)).collect())
    }

    /// Get all migration files from the migrations directory
    fn get_migration_files(&self) -> Result<Vec<(String, PathBuf)>, Box<dyn std::error::Error>> {
        let mut migrations = Vec::new();

        if !self.migrations_dir.exists() {
            return Ok(migrations);
        }

        let entries = fs::read_dir(&self.migrations_dir)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().map_or(false, |ext| ext == "sql") {
                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    migrations.push((file_name.to_string(), path));
                }
            }
        }

        migrations.sort();
        Ok(migrations)
    }

    /// Run migrations up to the latest version
    async fn up(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.init().await?;

        let executed = self.get_executed_migrations().await?;
        let files = self.get_migration_files()?;

        let mut ran_any = false;

        for (name, path) in files {
            if !executed.contains(&name) {
                let content = fs::read_to_string(&path)?;

                // Extract only the UP section
                if let Some(up_section) = content.split("-- UP").nth(1) {
                    let up_sql = up_section.split("-- DOWN").next().unwrap_or(up_section);
                    println!("Running migration: {}", name);
                    self.db_client.batch_execute(up_sql).await?;

                    self.db_client
                        .execute(
                            "INSERT INTO _schema_migrations (name) VALUES ($1)",
                            &[&name],
                        )
                        .await?;

                    ran_any = true;
                } else {
                    return Err(format!("Migration {} does not contain an UP section", name).into());
                }
            }
        }

        if ran_any {
            println!("✓ Migrations completed successfully");
        } else {
            println!("✓ No new migrations to run");
        }

        Ok(())
    }

    /// Rollback the last executed migration
    async fn down(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.init().await?;

        let executed = self.get_executed_migrations().await?;

        if executed.is_empty() {
            println!("✓ No migrations to rollback");
            return Ok(());
        }

        let last_migration = &executed[executed.len() - 1];
        let path = self.migrations_dir.join(last_migration);

        if !path.exists() {
            return Err(format!("Migration file not found: {}", last_migration).into());
        }

        let content = fs::read_to_string(&path)?;

        // Look for a DOWN section in the migration file
        if let Some(down_section) = content.split("-- DOWN").nth(1) {
            println!("Rolling back migration: {}", last_migration);
            self.db_client.batch_execute(down_section).await?;

            self.db_client
                .execute(
                    "DELETE FROM _schema_migrations WHERE name = $1",
                    &[&last_migration],
                )
                .await?;

            println!("✓ Migration rolled back successfully");
        } else {
            return Err("Migration file does not contain a DOWN section".into());
        }

        Ok(())
    }

    /// Create a new migration file with the given name
    fn create(&self, name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
        fs::create_dir_all(&self.migrations_dir)?;

        // Sanitize the migration name
        let clean_name = name
            .chars()
            .map(|c| {
                if c.is_alphanumeric() || c == '_' {
                    c
                } else {
                    '_'
                }
            })
            .collect::<String>();

        let timestamp = Local::now().format("%Y%m%d%H%M%S");
        let file_name = format!("{}_{}.sql", timestamp, clean_name);
        let file_path = self.migrations_dir.join(&file_name);

        let template = format!(
            "-- Migration: {}\n-- Created: {}\n\n-- UP\n-- Write your migration here\n\n-- DOWN\n-- Write rollback SQL here\n",
            clean_name,
            Local::now().format("%Y-%m-%d %H:%M:%S")
        );

        fs::write(&file_path, template)?;
        println!("✓ Created migration file: {}", file_name);

        Ok(file_path)
    }

    /// Reset the database by rolling back all migrations
    async fn reset(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.init().await?;

        let mut executed = self.get_executed_migrations().await?;
        executed.reverse();

        for migration in executed {
            let path = self.migrations_dir.join(&migration);

            if !path.exists() {
                return Err(format!("Migration file not found: {}", migration).into());
            }

            let content = fs::read_to_string(&path)?;

            if let Some(down_section) = content.split("-- DOWN").nth(1) {
                println!("Rolling back migration: {}", migration);
                self.db_client.batch_execute(down_section).await?;

                self.db_client
                    .execute(
                        "DELETE FROM _schema_migrations WHERE name = $1",
                        &[&migration],
                    )
                    .await?;
            } else {
                return Err(
                    format!("Migration {} does not contain a DOWN section", migration).into(),
                );
            }
        }

        println!("✓ Database reset successfully");
        Ok(())
    }

    /// Get the status of all migrations
    async fn status(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.init().await?;

        let executed = self.get_executed_migrations().await?;
        let files = self.get_migration_files()?;

        println!("\nMigration Status:");
        println!("{:<50} {}", "Migration", "Status");
        println!("{}", "-".repeat(60));

        for (name, _) in files {
            let status = if executed.contains(&name) {
                "✓ executed"
            } else {
                "⏳ pending"
            };
            println!("{:<50} {}", name, status);
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    let _ = dotenv();
    env_logger::init();

    let cli = Cli::parse();

    // Connect to the database
    let (client, connection) = tokio_postgres::connect(&cli.database_url, NoTls).await?;

    // Spawn the connection handler in the background
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Create the migrator
    let mut migrator = Migrator::new(client, cli.migrations_dir).await?;

    match cli.command {
        Commands::Up => migrator.up().await?,
        Commands::Down => migrator.down().await?,
        Commands::Create { name } => {
            migrator.create(&name)?;
        }
        Commands::Reset => migrator.reset().await?,
        Commands::Status => migrator.status().await?,
    }

    Ok(())
}
