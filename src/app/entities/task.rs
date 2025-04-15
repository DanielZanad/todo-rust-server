use chrono::{DateTime, Utc};

pub enum TaskState {
    Todo,
    InProgress,
    Completed,
}

pub struct Task {
    name: String,
    content: String,
    state: TaskState,
    done: bool,
    created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(name: String, content: String) -> Self {
        Task {
            name,
            content,
            state: TaskState::Todo,
            done: false,
            created_at: Utc::now(),
        }
    }

    // Getters
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn state(&self) -> &TaskState {
        &self.state
    }

    pub fn done(&self) -> bool {
        self.done
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    // Setters
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }

    pub fn set_state(&mut self, state: TaskState) {
        self.state = state;
    }

    pub fn set_done(&mut self, done: bool) {
        self.done = done;
    }
}
