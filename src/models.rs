use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::types::Train;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct GalleryEntry {
    pub id: i64,
    pub message: String,
    pub train: Train,
    pub submitter_name: Option<String>,
    pub submitted_at: chrono::NaiveDateTime,
    pub approved_at: Option<chrono::NaiveDateTime>,
    pub description: Option<String>,
}
