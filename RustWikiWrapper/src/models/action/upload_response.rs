use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UploadResponse {
    pub upload: UploadResult,
}

#[derive(Debug, Deserialize)]
pub struct UploadResult {
    pub result: String,
    pub filename: String,
    pub imageinfo: ImageInfo,
}

#[derive(Debug, Deserialize)]
pub struct ImageInfo {
    pub timestamp: String,
    pub user: String,
    pub userid: u64,
    pub size: u64,
    pub width: u32,
    pub height: u32,
    pub parsedcomment: String,
    pub comment: String,
    pub html: String,
    pub canonicaltitle: String,
    pub url: String,
    pub descriptionurl: String,
    pub sha1: String,
    pub metadata: Vec<Metadata>,
    pub commonmetadata: Vec<String>,
    pub extmetadata: ExtMetadata,
    pub mime: String,
    pub mediatype: String,
    pub bitdepth: u8,
}

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub value: i32,
}

#[derive(Debug, Deserialize)]
pub struct ExtMetadata {
    pub date_time: MetadataItem,
    pub object_name: MetadataItem,
    pub commons_metadata_extension: MetadataItem,
    pub categories: MetadataItem,
    pub assessments: MetadataItem,
}

#[derive(Debug, Deserialize)]
pub struct MetadataItem {
    pub value: String,
    pub source: String,
    pub hidden: Option<String>,
}