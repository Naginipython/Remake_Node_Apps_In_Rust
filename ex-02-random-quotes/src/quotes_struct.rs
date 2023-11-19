use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Quote {
    pub quote: String,
    pub author: String,
}