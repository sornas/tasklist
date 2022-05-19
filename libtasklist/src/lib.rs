use color_eyre::eyre::Result;

use crate::model::Database;

pub mod command;
pub mod model;

pub fn open() -> Result<Database> {
    let db_file = std::path::PathBuf::from("database.json");
    if db_file.exists() {
        let content = std::fs::read_to_string(db_file)?;
        Ok(serde_json::from_str(&content)?)
    } else {
        Ok(Database {
            routines: vec![],
            tasklists: vec![],
            tasks: vec![],
        })
    }
}

pub fn store(db: &Database) -> Result<()> {
    let content = serde_json::to_string(db)?;
    let db_file = std::path::PathBuf::from("database.json");
    std::fs::write(&db_file, &content)?;
    Ok(())
}
