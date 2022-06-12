use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

pub type Id = u64;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Database {
    pub routines: Vec<Routine>,
    pub tasklists: Vec<Tasklist>,
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
pub struct Tasklist {
    pub state: State,
    pub tasks: Vec<Id>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Task {
    // start: Option<>,
    // end: Option<>,
    // length: Option<>,
    // assigned: Option<User>,
    #[serde(with = "crate::serde::string")]
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

impl FromStr for State {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "notstarted" | "not-started" | "not_started" => Ok(State::NotStarted),
            "started" => Ok(State::Started),
            "paused" => Ok(State::Paused),
            "aborted" => Ok(State::Aborted),
            "done" => Ok(State::Done),
            s => Err(format!("unknown state {:?}", s)),
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                State::NotStarted => "not-started",
                State::Started => "started",
                State::Paused => "paused",
                State::Aborted => "aborted",
                State::Done => "done",
            }
        )
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Repetition {
    Manual,
    // Automatic [...]
}
