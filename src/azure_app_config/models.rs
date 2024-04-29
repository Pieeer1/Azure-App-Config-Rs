use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct KeyValueSet{
    items: Vec<KeyValue>
}
#[derive(Serialize, Deserialize)]
pub struct KeyValue{
    etag: String,
    key: String,
    label: Option<String>,
    content_type: Option<String>,
    value: String,
    tags: std::collections::HashMap<String, String>,
    locked: bool,
    last_modified: String
}

#[derive(Serialize, Deserialize)]
pub struct KeySet{
    items: Vec<Key>
}

#[derive(Serialize, Deserialize)]
pub struct Key{
    name: String
}

impl KeyValueSet {
    pub fn new(items: Vec<KeyValue>) -> Self {
        Self {
            items
        }
    }

    pub fn as_hash_map(&self) -> std::collections::HashMap<String, String> {
        let mut map = std::collections::HashMap::new();
        for kv in &self.items {
            map.insert(kv.key.clone(), kv.value.clone());
        }
        map
    }
}

impl KeySet {
    pub fn new(items: Vec<Key>) -> Self {
        Self {
            items
        }
    }
    pub fn as_vec(&self) -> Vec<String> {
        self.items.iter().map(|k| k.name.clone()).collect()
    }
}