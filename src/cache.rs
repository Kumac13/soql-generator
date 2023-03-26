use crate::helper::DynError;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheData {
    pub objects: Vec<String>,
    pub object_fields: HashMap<String, Vec<String>>,
    pub last_cached: DateTime<Utc>,
}

const CACHE_FILE: &str = "cache_data.json";
const CACHE_EXPIRATION_DAYS: i64 = 7;

pub fn save_cache_to_file(cache_data: &CacheData) -> Result<(), DynError> {
    let json = serde_json::to_string(cache_data)?;
    fs::write(CACHE_FILE, json)?;
    Ok(())
}

pub fn load_cache_from_file() -> Result<Option<CacheData>, DynError> {
    if Path::new(CACHE_FILE).exists() {
        let json = fs::read_to_string(CACHE_FILE)?;
        let cache_data: CacheData = serde_json::from_str(&json)?;

        let now = Utc::now();
        if (now - cache_data.last_cached).num_days() <= CACHE_EXPIRATION_DAYS {
            return Ok(Some(cache_data));
        }
    }
    Ok(None)
}
