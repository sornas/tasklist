use diesel::Queryable;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct Routine {
    pub id: i32,
    pub name: String,
    pub model: i32,
    // owner
    // repetition
}

pub struct ModelTasklist {
    pub id: i32,
    pub name: String,
    pub routine: i32,
}

pub struct User {
    pub id: i32,
    pub name: String,
    // password
    // email
}

#[derive(Queryable, Serialize)]
pub struct RegularTasklist {
    pub id: i32,
    pub name: String,
    pub state: String,
    pub belongs_to: i32,
    pub archived: bool,
}

#[derive(Queryable, Serialize)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub state: String,
}

#[derive(Queryable, Serialize)]
pub struct RegularPartOf {
    pub _id: i32,
    pub regular: i32,
    pub task: i32,
}

pub struct Assigned {
    pub _id: i32,
    pub task: i32,
    pub user: i32,
}

pub struct ModelMember {
    pub _id: i32,
    pub tasklist: i32,
    pub user: i32,
}

pub struct RoutineMember {
    pub _id: i32,
    pub routine: i32,
    pub user: i32,
}

pub struct RegularMember {
    pub _id: i32,
    pub tasklist: i32,
    pub user: i32,
}

#[derive(Queryable, Serialize)]
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
