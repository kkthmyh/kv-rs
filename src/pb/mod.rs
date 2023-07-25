use crate::{command_request::RequestData, value, CommandRequest, Hset, Kvpair, Value};

pub mod abi;
// 实现创建CommandRequest
impl CommandRequest {
    pub fn new_hset(table: impl Into<String>, key: impl Into<String>, value: Value) -> Self {
        Self {
            request_data: Some(RequestData::Hset(Hset {
                table: table.into(),
                pair: Some(Kvpair::new_kvpair(key.into(), value.into())),
            })),
        }
    }
}

// 实现创建keypair
impl Kvpair {
    pub fn new_kvpair(key: impl Into<String>, value: Value) -> Self {
        Self {
            key: key.into(),
            value: Some(value),
        }
    }
}

// 为Value实现String类型转换
impl From<String> for Value {
    fn from(value: String) -> Self {
        Value {
            value: Some(value::Value::String(value)),
        }
    }
}

// 为Value实现&str类型转换
impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value {
            value: Some(value::Value::String(value.into())),
        }
    }
}
