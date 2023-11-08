use std::fmt::Pointer;
use std::fs::OpenOptions;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::value::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Node {
    pub name: String,
    pub describe: String,
    pub next: Vec<Node>,
    pub input: Option<Value>,
    pub output: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeRunResult {
    pub message: Option<String>,
    pub result: Option<Value>,
}

