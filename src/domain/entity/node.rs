use crate::domain::value_object::ID;

#[derive(Debug)]
pub struct Node {
    id: ID,
    label: String,
}

impl Node {
    pub fn new(id: ID, label: &str) -> Self {
        Self {
            id,
            label: label.to_string(),
        }
    }
}
