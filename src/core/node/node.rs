use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::value::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Node {
    pub uid: String,
    pub name: String,
    pub describe: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeRunResult {
    pub message: Option<String>,
    pub result: Option<Value>,
}
