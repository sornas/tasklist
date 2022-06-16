use color_eyre::eyre::{anyhow, Result};
use diesel::prelude::*;
use tasklist_lib::model;

use crate::db;

#[derive(Clone, Debug, Queryable)]
pub struct Routine {
    pub id: i32,
    pub name: String,
    pub model: i32,
    // owner
    // repetition
}

impl Routine {
    pub fn to_model(self, tasklists: Vec<i32>) -> Result<model::Routine> {
        Ok(model::Routine {
            id: self.id,
            model: self.model,
            name: self.name,
            repetition: model::Repetition::Manual,
            tasklists,
        })
    }

    pub fn tasklists(&self, connection: &SqliteConnection) -> QueryResult<Vec<i32>> {
        use db::schema::tasklist::dsl;

        dsl::tasklist
            .filter(dsl::routine_id.eq(self.id))
            .select(dsl::id)
            .load(connection)
    }
}

#[derive(Debug, Queryable)]
pub struct ModelTasklist {
    pub id: i32,
    pub routine: i32,
}

#[derive(Debug, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    // password
    // email
}

#[derive(Clone, Debug, Queryable)]
pub struct RegularTasklist {
    pub id: i32,
    pub name: String,
    pub state: String,
    pub routine_id: i32,
    pub archived: bool,
}

impl RegularTasklist {
    pub fn to_model(self, tasks: Vec<i32>) -> Result<model::Tasklist> {
        Ok(model::Tasklist {
            id: self.id,
            name: self.name,
            // NOTE this type hint is required. weird
            state: self.state.parse().map_err(|e: String| anyhow!(e))?,
            tasks,
        })
    }
}

#[derive(Clone, Debug, Queryable)]
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

#[derive(Debug, Queryable)]
pub struct RegularPartOf {
    pub _id: i32,
    pub regular: i32,
    pub task: i32,
}

#[derive(Debug, Queryable)]
pub struct Assigned {
    pub _id: i32,
    pub task: i32,
    pub user: i32,
}

#[derive(Debug, Queryable)]
pub struct ModelMember {
    pub _id: i32,
    pub tasklist: i32,
    pub user: i32,
}

#[derive(Debug, Queryable)]
pub struct RoutineMember {
    pub _id: i32,
    pub routine: i32,
    pub user: i32,
}

#[derive(Debug, Queryable)]
pub struct RegularMember {
    pub _id: i32,
    pub tasklist: i32,
    pub user: i32,
}

#[derive(Debug, Queryable)]
pub struct ModelPartof {
    pub _id: i32,
    pub model: i32,
    pub task: i32,
}

pub mod insert {
    use diesel::prelude::*;
    use tasklist_lib::model;

    use crate::db;

    use db::schema::model as model_;
    use db::schema::*;

    macro_rules! impl_insert {
        ($table:expr) => {
            pub fn insert(&self, connection: &SqliteConnection) -> QueryResult<()> {
                diesel::insert_into($table)
                    .values(self)
                    .execute(connection)?;
                Ok(())
            }

            pub fn insert_and_id(&self, connection: &SqliteConnection) -> QueryResult<i32> {
                self.insert(connection)?;
                db::last_insert_rowid(connection)
            }
        };
    }

    #[derive(Debug, Insertable)]
    #[table_name = "task"]
    pub struct Task {
        pub name: String,
        pub state: String,
    }

    impl Task {
        impl_insert!(task::table);
    }

    impl From<model::Task> for Task {
        fn from(other: model::Task) -> Self {
            Self {
                name: other.name,
                state: other.state.to_string(),
            }
        }
    }

    #[derive(Debug, Insertable)]
    #[table_name = "tasklist"]
    pub struct RegularTasklist<'a> {
        pub name: &'a str,
        pub state: &'a str,
        pub routine_id: i32,
    }

    impl<'a> RegularTasklist<'a> {
        impl_insert!(tasklist::table);
    }

    #[derive(Debug, Insertable)]
    #[table_name = "task_partof_regular"]
    pub struct TaskPartofRegular {
        pub regular: i32,
        pub task: i32,
    }

    impl TaskPartofRegular {
        impl_insert!(task_partof_regular::table);
    }

    #[derive(Debug, Insertable)]
    #[table_name = "task_partof_model"]
    pub struct TaskPartofModel {
        pub model: i32,
        pub task: i32,
    }

    impl TaskPartofModel {
        impl_insert!(task_partof_model::table);
    }

    #[derive(Debug, Insertable)]
    #[table_name = "routine"]
    pub struct Routine<'a> {
        pub name: &'a str,
        pub model: i32,
    }

    impl<'a> Routine<'a> {
        impl_insert!(routine::table);
    }

    #[derive(Debug, Insertable)]
    #[table_name = "model_"]
    pub struct ModelTasklist {
        pub routine: i32,
    }

    impl ModelTasklist {
        impl_insert!(model_::table);
    }
}
