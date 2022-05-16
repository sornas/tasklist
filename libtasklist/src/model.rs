use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Routine {
    pub name: String,
    pub repetition: Repetition,
    pub model: TaskList,
    pub task_lists: Vec<TaskList>,
    // owner: User,
    // members: Vec<User>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TaskList {
    pub tasks: Vec<Task>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Task {
    // start: Option<>,
    // end: Option<>,
    // length: Option<>,
    // assigned: Option<User>,
    pub state: TaskState,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum TaskState {
    NotStarted,
    Started,
    Paused,
    Aborted,
    Done,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Repetition {
    Manual,
    // Automatic [...]
}
