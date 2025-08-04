use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CommandType {
    Add,
    List,
    Remove,
    Update,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reminder {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub due_date: String,
    pub timestamp: String,
}

impl Reminder {
    pub fn new(id: i32, title: String, description: String, due_date: String, timestamp: String) -> Self {
        Self { id, title, description, due_date, timestamp }
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }
}
