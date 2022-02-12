use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct Data {
    pub title: String,
    pub date: String,
    pub description: String,
    pub media: Option<Vec<String>>,
    pub category: String,
    pub button: Option<[String; 2]>,
}
