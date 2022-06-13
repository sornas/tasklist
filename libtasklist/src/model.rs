use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

pub type Id = i32;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Routine {
    pub name: String,
    pub repetition: Repetition,
    pub model: Id,
    pub tasklists: Vec<Id>,
    // owner: User,
    // members: Vec<User>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Tasklist {
    pub id: i32,
    pub name: String,
    #[serde(with = "crate::serde::string")]
    pub state: State,
    pub tasks: Vec<Id>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Task {
    // start: Option<>,
    // end: Option<>,
    // length: Option<>,
    // assigned: Option<User>,
    pub name: String,
    #[serde(with = "crate::serde::string")]
    pub state: State,
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
