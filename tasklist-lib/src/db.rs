use diesel::prelude::*;

pub mod model;
pub mod schema;

pub mod tasklists;

pub fn last_insert_rowid(connection: &SqliteConnection) -> QueryResult<i32> {
    no_arg_sql_function!(
        last_insert_rowid,
        diesel::sql_types::Integer,
        "Represents the SQL last_insert_row() function"
    );

    diesel::select(last_insert_rowid).get_result::<i32>(connection)
}
