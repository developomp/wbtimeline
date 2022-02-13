use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct Data {
    pub date: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub subcategory: String,
    pub media: Option<Vec<String>>,
    pub button: Option<[String; 2]>,
}
