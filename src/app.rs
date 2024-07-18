use std::error;

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct TodoApp {
    // TODO: decide on what state needs to be tracked throughout the TUI app
    // for things such as viewing current todos and editing/creating new ones
    // users should be able to press enter on the main list of todos to see a specific
    // todo/task details
    pub running: bool,
}

impl Default for TodoApp {
    fn default() -> Self {
        Self { running: true }
    }
}

impl TodoApp {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn exit(&mut self) {
        self.running = false;
    }
}
