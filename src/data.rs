use serde::de::{self, Deserializer, Visitor};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Default, Serialize)]
pub struct Board {
    pub name: String,
    pub threads: Vec<Thread>,
}

impl<'de> Visitor<'de> for Board {
    type Value = Board;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{:?}", self)
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut output = Board::default();
        while let Some((key, value)) = map.next_entry::<String, String>()? {
            match key.as_str() {
                "name" => output.name = value,
                _ => { /* TODO: Add an error */ }
            }
        }
        Ok(output)
    }
}

impl<'de> Deserialize<'de> for Board {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(Board::default())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Thread {
    pub title: String,
    pub posts: Vec<Post>,
}

#[derive(Debug, Serialize, Clone)]
pub struct Post {
    pub publication: SystemTime,
    pub text: String,
    pub img: String,
}

impl Default for Post {
    fn default() -> Self {
        Post {
            publication: SystemTime::now(),
            text: String::default(),
            img: String::default()
        }
    }
}

impl<'de> Visitor<'de> for Post {
    type Value = Post;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{:?}", self)
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut output = Post::default();
        while let Some((key, value)) = map.next_entry::<String, String>()? {
            match key.as_str() {
                "text" => output.text = value,
                // TODO: Images
                _ => { /* TODO: Add an error */ }
            }
        }
        Ok(output)
    }
}

impl<'de> Deserialize<'de> for Post {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(Post::default())
    }
}
