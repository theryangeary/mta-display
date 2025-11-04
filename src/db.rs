use std::str::FromStr;

use anyhow::Result;
use sqlx::{
    migrate::MigrateDatabase,
    sqlite::{SqliteConnectOptions, SqlitePool},
    ConnectOptions, FromRow, Row, Sqlite,
};

use crate::{models::GalleryEntry, types::Train};

#[async_trait::async_trait]
pub trait Database: Send + Sync {
    async fn create_gallery_entry(
        &self,
        message: &str,
        train: Train,
        submitter_name: Option<&str>,
        description: Option<&str>,
    ) -> Result<()>;

    async fn approve_gallery_entry(&self, entry_id: i64) -> Result<()>;

    async fn reject_gallery_entry(&self, entry_id: i64) -> Result<()>;

    async fn list_pending_gallery_entries(&self) -> Result<Vec<GalleryEntry>>;

    async fn list_approved_gallery_entries(&self) -> Result<Vec<GalleryEntry>>;
}
pub struct SqliteDatabase {
    pub pool: SqlitePool,
}

impl SqliteDatabase {
    pub async fn new(database_url: &str) -> Result<Self> {
        if !Sqlite::database_exists(database_url).await.unwrap_or(false) {
            Sqlite::create_database(database_url).await?;
        }

        let options =
            SqliteConnectOptions::from_str(database_url)?.log_statements(tracing::log::LevelFilter::Trace).foreign_keys(true);

        let pool = SqlitePool::connect_with(options).await?;

        let db = SqliteDatabase { pool };
        db.migrate().await?;

        Ok(db)
    }

    async fn migrate(&self) -> Result<()> {
        sqlx::migrate!("./migrations").run(&self.pool).await?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl Database for SqliteDatabase {
    async fn create_gallery_entry(
        &self,
        message: &str,
        train: Train,
        submitter_name: Option<&str>,
        description: Option<&str>,
    ) -> Result<()> {
        sqlx::query(
            "INSERT INTO gallery_entries (message, train, submitter_name, description) VALUES (?, ?, ?, ?)",
        )
        .bind(message)
        .bind(train as Train)
        .bind(submitter_name)
        .bind(description)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn approve_gallery_entry(&self, entry_id: i64) -> Result<()> {
        sqlx::query("UPDATE gallery_entries SET approved_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(entry_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn reject_gallery_entry(&self, entry_id: i64) -> Result<()> {
        sqlx::query("DELETE FROM gallery_entries WHERE id = ?")
            .bind(entry_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
    
    async fn list_approved_gallery_entries(&self) -> Result<Vec<GalleryEntry>> {
        sqlx::query_as::<_, GalleryEntry>(
            "SELECT id, message, train, submitter_name, submitted_at, approved_at, description FROM gallery_entries WHERE approved_at IS NOT NULL",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.into())  
    }

    async fn list_pending_gallery_entries(&self) -> Result<Vec<GalleryEntry>> {
        sqlx::query_as::<_, GalleryEntry>(
            "SELECT id, message, train, submitter_name, submitted_at, approved_at, description FROM gallery_entries WHERE approved_at IS NULL",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.into())
    }
}
