pub type AnyhowError = anyhow::Error;
pub type LogLevel = log::Level;

#[uniffi::remote(Object)]
pub struct AnyhowError;

#[uniffi::remote(Enum)]
pub enum LogLevel {
    Error = 1,
    Warn = 2,
    Info = 3,
    Debug = 4,
    Trace = 5,
}

pub mod remote {
    use url::Url;

    #[uniffi::remote(Object)]
    pub struct Url;

    pub type Count = usize;

    uniffi::custom_type!(Count, u64, {
        remote,
        lower: |count| count as u64,
        try_lift: |value| Ok(value as usize),
    });

    use serde_json::Value;

    pub type Json = Value;

    uniffi::custom_type!(Json, String, {
        remote,
        lower: |json| json.to_string(),
        try_lift: |value| Ok(serde_json::from_str(&value)?),
    });
}

pub mod consumer {
    // Refers to the remote type by its original path, not the path of the remote item
    use url::Url;

    #[uniffi::export]
    pub fn parse_url(url: Url) -> Url {
        url
    }
}
