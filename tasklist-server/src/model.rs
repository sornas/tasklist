use diesel::Queryable;
use serde::Serialize;

use crate::schema::tasks;

#[derive(Queryable)]
pub struct Model {
    pub id: i32,
}

#[derive(Queryable)]
pub struct Routine {
    pub id: i32,
    pub name: String,
    pub model: i32,
}

#[derive(Queryable)]
pub struct TasklistPartof {
    pub _id: i32,
    pub tasklist: i32,
    pub task: i32,
}

#[derive(Queryable)]
pub struct Tasklist {
    pub id: i32,
    pub name: String,
    pub state: String,
    pub belongs_to: i32,
}

#[derive(Queryable, Serialize)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub state: String,
}

#[derive(Insertable)]
#[table_name = "tasks"]
pub struct NewTask<'a> {
    pub name: &'a str,
    pub state: &'a str,
}
