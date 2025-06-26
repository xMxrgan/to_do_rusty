use crate::tasks::Task;
use rusqlite::{Connection, Result, params};

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("tudu.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT,
            priority INTEGER CHECK(priority >= 1 AND priority <= 5),
            status TEXT DEFAULT 'active'
        )",
        [],
    )?;

    Ok(conn)
}

pub fn insert_task(conn: &Connection, task: &Task) -> Result<()> {
    conn.execute(
        "INSERT INTO tasks (name, description, priority, status) VALUES (?1, ?2, ?3, ?4)",
        params![task.name, task.description, task.priority, task.status],
    )?;
    Ok(())
}

pub fn get_all_tasks(conn: &Connection) -> Result<Vec<Task>> {
    let mut stmt = conn.prepare("SELECT id, name, description, priority, status FROM tasks")?;
    let task_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            priority: row.get(3)?,
            status: row.get(4)?,
        })
    })?;

    let mut tasks = Vec::new();
    for task in task_iter {
        tasks.push(task?);
    }

    Ok(tasks)
}
