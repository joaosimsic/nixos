pub struct SessionMetadata {
    pub _name: String,
    pub cwd: String,
    pub tabs: usize,
    pub branch: Option<String>,
    pub commands: Vec<String>,
}

impl SessionMetadata {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            cwd: "unknown".to_string(),
            tabs: 0,
            branch: None,
            commands: Vec::new(),
        }
    }
}
