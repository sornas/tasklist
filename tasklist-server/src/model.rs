use diesel::Queryable;

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
    pub done: bool,
    pub belongs_to: i32,
}

#[derive(Queryable)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub done: bool,
}

#[derive(Insertable)]
#[table_name = "tasks"]
pub struct NewTask<'a> {
    pub name: &'a str,
}
