#[derive(Clone, PartialEq)]
pub struct Data {
    pub title: String,
    pub date: String,
    pub description: String,
    pub media: Option<Vec<String>>,
}
