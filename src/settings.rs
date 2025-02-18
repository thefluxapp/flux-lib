use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct HttpSettings {
    pub endpoint: String,
}

#[derive(Deserialize, Clone)]
pub struct DBSettings {
    pub endpoint: String,
}
