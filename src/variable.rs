pub struct Variable {
    pub vec: Vec<[String;2]>
}

impl Variable {
    pub fn load() -> Self {
        Self { vec: vec![[String::new(), String::new()]] }
    }
}
