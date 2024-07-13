use super::ID;

#[derive(Debug)]
pub struct IdCollection {
    value: Vec<ID>,
}

impl IdCollection {
    pub fn new() -> Self {
        Self { value: Vec::new() }
    }
}
