use serde::{Serialize, Deserialize}
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]  //implement traits
pub struct Message {
    pub channel: String,
    pub content: String,
    pub metadata: HashMap<String, String>
    pub timestamp: DateTime<Utc>,

}

impl Message {

    pub fn new(channel: String, content: String) ->Self { //static constructor method
        Message {

            channel,
            content,
            metadata:HashMap::new(),
            timestamp: Utc::now()

        }
    }

    pub fn with_metadata(mut self, key: &str, value: &str) -> Self { //instance method for modifying metadata
        self.metadata.insert(key.to_string(), value.to_string());
        self

    }

}
