use ::tasklists::model;
use color_eyre::eyre::{anyhow, Result};
use diesel::Queryable;

use super::schema::{tasklists, tasks};

#[derive(Queryable)]
pub struct Routine {
    pub id: i32,
    pub name: String,
    pub model: i32,
    // owner
    // repetition
}

#[derive(Queryable)]
pub struct ModelTasklist {
    pub id: i32,
    pub routine: i32,
}

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    // password
    // email
}

#[derive(Clone, Queryable, Identifiable, Associations)]
#[table_name = "tasklists"] // TODO we should have different tables for regular and model?
pub struct RegularTasklist {
    pub id: i32,
    pub name: String,
    pub state: String,
    pub belongs_to: i32,
    pub archived: bool,
}

impl RegularTasklist {
    pub fn to_model(self, tasks: Vec<i32>) -> Result<model::Tasklist> {
        Ok(model::Tasklist {
            name: self.name,
            // NOTE this type hint is required. weird
            state: self.state.parse().map_err(|e: String| anyhow!(e))?,
            tasks,
        })
    }
}

#[derive(Clone, Queryable, Identifiable, Associations)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub state: String,
}

impl Task {
    pub fn to_model(self) -> Result<model::Task> {
        Ok(model::Task {
            name: self.name,
            state: self.state.parse().map_err(|e: String| anyhow!(e))?,
        })
    }
}

#[derive(Queryable)]
#[diesel(belongs_to(RegularTasklist, foreign_key = "regular"))]
#[diesel(belongs_to(Task, foreign_key = "task"))]
pub struct RegularPartOf {
    pub _id: i32,
    pub regular: i32,
    pub task: i32,
}

#[derive(Queryable)]
pub struct Assigned {
    pub _id: i32,
    pub task: i32,
    pub user: i32,
}

#[derive(Queryable)]
pub struct ModelMember {
    pub _id: i32,
    pub tasklist: i32,
    pub user: i32,
}

#[derive(Queryable)]
pub struct RoutineMember {
    pub _id: i32,
    pub routine: i32,
    pub user: i32,
}

#[derive(Queryable)]
pub struct RegularMember {
    pub _id: i32,
    pub tasklist: i32,
    pub user: i32,
}

#[derive(Queryable)]
pub struct ModelPartof {
    pub _id: i32,
    pub model: i32,
    pub task: i32,
}

pub mod insert {
    pub use crate::schema::*;

    #[derive(Insertable)]
    #[table_name = "tasks"]
    pub struct Task<'a> {
        pub name: &'a str,
        pub state: &'a str,
    }
}
