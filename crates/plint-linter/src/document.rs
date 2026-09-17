pub struct Document {
    pub name: String,
    pub content: String,
}

impl Document {
    pub fn new(name: String, content: String) -> Self {
        Document { name, content }
    }
}
