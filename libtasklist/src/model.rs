use serde::{Deserialize, Serialize};

pub type Id = u64;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Database {
    pub routines: Vec<Routine>,
    pub tasklists: Vec<TaskList>,
    pub tasks: Vec<Task>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Routine {
    pub name: String,
    pub repetition: Repetition,
    pub model: Id,
    pub task_lists: Vec<Id>,
    // owner: User,
    // members: Vec<User>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Metadata {
    pub next_routine: Id,
    pub next_task: Id,
    pub next_tasklist: Id,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TaskList {
    pub state: State,
    pub tasks: Vec<Id>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Task {
    // start: Option<>,
    // end: Option<>,
    // length: Option<>,
    // assigned: Option<User>,
    pub state: State,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum State {
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
