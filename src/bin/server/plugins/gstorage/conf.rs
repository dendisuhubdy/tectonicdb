use std::error::Error;

use settings::{key_or_default_parse, key_or_default, key_or_none};

#[derive(Debug)]
pub struct GStorageConfig {
    /// google cloud oauth token
    pub oauth_token: Option<String>,
    /// name of the bucket
    /// i.e. gs://tick-data
    pub bucket_name: String,
    /// folder name in bucket
    /// gs://tick-data/{folder}
    pub folder: String,
    /// upload interval in seconds
    pub interval: u64,
    /// remove file when it's done?
    pub remove: bool,
    /// data collection backend - if you don't know just ignore
    pub dcb_url: Option<String>,
    /// tags to insert in data uploaded to the DCB
    pub dcb_metadata_tags: Vec<String>,
}

fn parse_dcb_metadata_tags() -> Vec<String> {
    key_or_default("DCB_METADATA_TAGS", "")
        .split(',')
        .map(String::from)
        .collect()
}

impl GStorageConfig {
    /// Creates a new `GStorageConfig` from environment variables, filling in the remaining values with
    /// defaults if not available.
    pub fn new() -> Result<Self, Box<Error>> {
        Ok(GStorageConfig {
            oauth_token: key_or_none("GCLOUD_OAUTH_TOKEN"),
            bucket_name: key_or_default("GCLOUD_BUCKET_NAME", "tick_data"),
            folder: key_or_default("GCLOUD_FOLDER", ""),
            interval: key_or_default_parse("GCLOUD_UPLOAD_INTERVAL", 3600)?,
            remove: key_or_default_parse("GCLOUD_REMOVE_ON_UPLOAD", true)?,
            dcb_url: key_or_none("DCB_URL"),
            dcb_metadata_tags: parse_dcb_metadata_tags(),
        })
    }
}

#[test]
fn dcb_metadata_tags_parsing() {
    let sample_env = "foo,bar,key:value,test2";
    let parsed: Vec<String> = sample_env
        .split(',')
        .map(String::from)
        .collect();

    let expected: Vec<String> = ["foo", "bar", "key:value", "test2"]
        .into_iter()
        .map(|s| String::from(*s))
        .collect();

    assert_eq!(parsed, expected);
}
