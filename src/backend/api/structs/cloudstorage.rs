use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SystemEntry {
    #[serde(rename = "uniqueFilename")]
    pub unique_filename: String,
    pub filename: String,
    pub hash: String,
    pub hash256: String,
    pub length: usize,
    #[serde(rename = "contentType")]
    pub content_type: String,
    pub uploaded: String,
    #[serde(rename = "storageType")]
    pub storage_type: String,
    #[serde(rename = "doNotCache")]
    pub do_not_cache: bool,
}