use serde::{Deserialize, Serialize};

use crate::model::State;

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct MarkTask {
    pub state: Option<State>,
    pub name: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct MarkTasklist {
    pub state: Option<State>,
}
