use serde::{Deserialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct UploadResponse {
    pub upload: UploadResult,
}

// Struct for the upload result
#[derive(Debug, Deserialize)]
pub struct UploadResult {
    pub result: String,    // The result of the upload attempt (e.g., "Success", "Warning", "Failure")
    pub filename: Option<String>, // The filename of the uploaded file (if successful)
    pub warnings: Option<HashMap<String, String>>, // Warnings (if any)
    pub error: Option<HashMap<String, String>>,    // Errors (if any)
    // A placeholder for upload result structure
}