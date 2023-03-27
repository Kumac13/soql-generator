use crate::helper::DynError;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheData {
    pub objects: Vec<String>,
    pub object_fields: HashMap<String, Vec<String>>,
    pub last_cached: DateTime<Utc>,
}

const CACHE_EXPIRATION_DAYS: i64 = 7;

pub fn save_cache_to_file(
    cache_data: &CacheData,
    cache_data_path: &PathBuf,
) -> Result<(), DynError> {
    let json = serde_json::to_string(cache_data)?;
    fs::write(cache_data_path, json)?;
    Ok(())
}

pub fn load_cache_from_file(cache_data_path: &PathBuf) -> Result<Option<CacheData>, DynError> {
    if Path::new(&cache_data_path).exists() {
        let json = fs::read_to_string(cache_data_path)?;
        let cache_data: CacheData = serde_json::from_str(&json)?;

        let now = Utc::now();
        if (now - cache_data.last_cached).num_days() <= CACHE_EXPIRATION_DAYS {
            return Ok(Some(cache_data));
        }
    }
    Ok(None)
}
