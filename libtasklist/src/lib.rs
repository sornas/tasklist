use color_eyre::eyre::Result;

pub mod model;

// B)
pub type Database = Vec<model::Routine>;

pub fn open() -> Result<Database> {
    let db_file = std::path::PathBuf::from("routines.json");
    if db_file.exists() {
        let content = std::fs::read_to_string(db_file)?;
        Ok(serde_json::from_str(&content)?)
    } else {
        Ok(vec![])
    }
}

pub fn store(db: Database) -> Result<()> {
    let content = serde_json::to_string(&db)?;
    let db_file = std::path::PathBuf::from("routines.json");
    std::fs::write(&db_file, &content)?;
    Ok(())
}
