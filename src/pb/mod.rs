use crate::{
    command_request::RequestData, value, CommandRequest, Hget, Hgetall, Hset, Kvpair, Value,
};

pub mod abi;
// 实现创建CommandRequest
impl CommandRequest {
    pub fn new_hset(table: impl Into<String>, key: impl Into<String>, value: Value) -> Self {
        Self {
            request_data: Some(RequestData::Hset(Hset {
                table: table.into(),
                pair: Some(Kvpair::new(key.into(), value.into())),
            })),
        }
    }

    pub fn new_hget(table: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            request_data: Some(RequestData::Hget(Hget {
                table: table.into(),
                key: key.into(),
            })),
        }
    }

    pub fn new_hgetall(table: impl Into<String>) -> Self {
        Self {
            request_data: Some(RequestData::Hgetall(Hgetall {
                table: table.into(),
            })),
        }
    }
}

// 实现创建keypair
impl Kvpair {
    pub fn new(key: impl Into<String>, value: Value) -> Self {
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

// 为Value实现i64的类型转换
impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self {
            value: Some(value::Value::Integer(value)),
        }
    }
}
