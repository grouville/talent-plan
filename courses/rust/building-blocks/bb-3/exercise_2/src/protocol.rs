use serde::{Deserialize, Serialize, Serializer, Deserializer};

#[derive(Debug, PartialEq)]
pub enum RedisMessage {
    Ping,
    Pong,
}

impl Serialize for RedisMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            RedisMessage::Ping => serializer.serialize_str("*1\r\n$4\r\nPING\r\n"),
            RedisMessage::Pong => serializer.serialize_str("+PONG\r\n"),
        }
    }
}

impl<'de> Deserialize<'de> for RedisMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        if s == "*1\r\n$4\r\nPING\r\n" {
            Ok(RedisMessage::Ping)
        } else if s == "+PONG\r\n" {
            Ok(RedisMessage::Pong)
        } else {
            Err(serde::de::Error::custom("Invalid message"))
        }
    }
}

