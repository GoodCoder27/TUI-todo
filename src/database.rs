use rusqlite::{params, Connection, Result};
use std::collections::HashMap;

pub fn initialize_database(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY,
            description TEXT NOT NULL,
            is_done BOOLEAN NOT NULL
        )",
        [],
    )?;
    Ok(())
}

pub fn load_tasks(conn: &Connection) -> Result<HashMap<bool, Vec<String>>> {
    let mut stmt = conn.prepare("SELECT description, is_done FROM tasks")?;
    let task_iter = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, bool>(1)?))
    })?;

    let mut tasks: HashMap<bool, Vec<String>> = HashMap::new();
    tasks.insert(false, Vec::new());
    tasks.insert(true, Vec::new());

    for task in task_iter {
        let (description, is_done) = task?;
        tasks.get_mut(&is_done).unwrap().push(description);
    }

    Ok(tasks)
}

pub fn task_exists(conn: &Connection, description: &str) -> Result<bool> {
    let mut stmt = conn.prepare("SELECT EXISTS(SELECT 1 FROM tasks WHERE description = ?1)")?;
    let exists: bool = stmt.query_row(params![description], |row| row.get(0))?;
    Ok(exists)
}

pub fn add_task(conn: &mut Connection, description: &str, is_done: bool) -> Result<()> {
    let tx = conn.transaction()?;
    tx.execute(
        "INSERT INTO tasks (description, is_done) VALUES (?1, ?2)",
        (description, is_done),
    )?;
    tx.commit()
}

pub fn remove_task(conn: &mut Connection, description: &str) -> Result<()> {
    let tx = conn.transaction()?;
    tx.execute("DELETE FROM tasks WHERE description = ?1", [description])?;
    tx.commit()
}

pub fn complete_task(conn: &mut Connection, description: &str) -> Result<()> {
    let tx = conn.transaction()?;
    tx.execute(
        "UPDATE tasks SET is_done = TRUE WHERE description = ?1",
        [description],
    )?;
    tx.commit()
}

pub fn uncomplete_task(conn: &mut Connection, description: &str) -> Result<()> {
    let tx = conn.transaction()?;
    tx.execute(
        "UPDATE tasks SET is_done = FALSE WHERE description = ?1",
        [description],
    )?;
    tx.commit()
}
