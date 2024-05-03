use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct KeyValueSet{
    pub items: Vec<KeyValue>
}
#[derive(Serialize, Deserialize, Clone)]
pub struct KeyValue{
    pub etag: String,
    pub key: String,
    pub label: Option<String>,
    pub content_type: Option<String>,
    pub value: String,
    pub tags: std::collections::HashMap<String, String>,
    pub locked: bool,
    pub last_modified: String
}

#[derive(Serialize, Deserialize)]
pub struct KeySet{
    pub items: Vec<Key>
}

#[derive(Serialize, Deserialize)]
pub struct Key{
    pub name: String
}

#[derive(Serialize, Deserialize)]
pub struct FeatureFlagSet{
    pub items: Vec<FeatureFlag>
}

#[derive(Serialize, Deserialize)]
pub struct FeatureFlag{
    pub id: String,
    pub enabled: bool
}

impl FeatureFlagSet{
    pub fn from_key_value_set(kv_set: KeyValueSet) -> Self{
        let mut flags = Vec::new();
        for kv in kv_set.items{
            flags.push(FeatureFlag::from_key_value(kv));
        }
        Self{
            items: flags
        }
    }
    
    pub fn as_hash_map(&self) -> std::collections::HashMap<String, bool>{
        let mut map = std::collections::HashMap::new();
        for flag in &self.items{
            map.insert(flag.id.clone(), flag.enabled);
        }
        map
    }
}

impl FeatureFlag{
    pub fn from_key_value(kv: KeyValue) -> Self{
        serde_json::from_str(kv.value.as_str()).unwrap()
    }
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