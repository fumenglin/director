use std::fmt::Error;
use std::string::ToString;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use axum::{body::Body, response::Response};

extern crate serde_json;

pub const CODE_SUCCESS: i8 = 0;
pub const CODE_FAIL: i8 = 1;

pub const MSG_SUCCESS: &str = "成功";
pub const MSG_FAIL: &str = "失败";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResultVO<T> {
    pub status: bool,
    pub code: Option<i8>,
    pub msg: Option<String>,
    pub body: Option<T>,
}

impl<T> ResultVO<T>
    where
        T: Serialize + DeserializeOwned + Clone
{
    pub fn from_result(res: Result<T, Error>) -> Self {
        if res.is_ok() {
            Self {
                status: true,
                code: Some(CODE_SUCCESS),
                msg: Some(MSG_SUCCESS.to_string()),
                body: res.clone().ok(),
            }
        } else {
            let err = res.clone().err();
            let msg = match err {
                None => MSG_FAIL.to_string(),
                Some(_) => err.unwrap().to_string()
            };
            Self {
                status: false,
                code: Some(CODE_FAIL),
                msg: Some(msg),
                body: None,
            }
        }
    }


    pub fn from_msg_success(msg: &str) -> Self {
        Self {
            status: true,
            code: Some(CODE_SUCCESS),
            msg: Some(msg.to_string()),
            body: None,
        }
    }
    pub fn from_msg_fail(msg: &str) -> Self {
        Self {
            status: false,
            code: Some(CODE_FAIL),
            msg: Some(msg.to_string()),
            body: None,
        }
    }
    pub fn resp_json(&self) -> Response<Body> {
        Response::builder()
            .extension(|| {})
            .header("Access-Control-Allow-Origin", "*")
            .header("Cache-Control", "no-cache")
            .header("Content-Type", "text/json;charset=UTF-8")
            .body(Body::from(self.to_string()))
            .unwrap()
    }
}

impl<T> ToString for ResultVO<T>
    where
        T: Serialize + DeserializeOwned + Clone
{
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
