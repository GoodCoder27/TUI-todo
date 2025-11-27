use std::{collections::HashMap, io::stdout};

use crossterm::{
    cursor::{self, MoveToColumn, SavePosition},
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::Print,
    terminal::{self, ClearType},
};
use rusqlite::Connection;

use crate::border;
use crate::database;

pub fn create_task(width: u16, conn: &mut Connection) {
    let mut stdout = stdout();
    let mut input = String::new();
    let x: u16 = (width / 3 * 2) - 1;

    execute!(stdout, SavePosition).unwrap();

    loop {
        if let Ok(true) = event::poll(std::time::Duration::from_millis(100)) {
            if let Ok(Event::Key(KeyEvent { code, .. })) = event::read() {
                match code {
                    KeyCode::Enter => {
                        if !database::task_exists(conn, &input.to_string())
                            .expect("i don't what happened here")
                        {
                            database::add_task(conn, &input.to_string(), false)
                                .expect("Failed to add task");

                            execute!(
                                stdout,
                                cursor::RestorePosition,
                                Print(format!(" [ ] {}", input))
                            )
                            .unwrap();
                        } else {
                            execute!(
                                stdout,
                                MoveToColumn(1),
                                Print(" ".repeat(x as usize)), // Print spaces up to `x`
                                MoveToColumn(1),
                            )
                            .unwrap();
                        }
                        input.clear(); // Clear input after submission
                        break;
                    }
                    KeyCode::Backspace => {
                        input.pop(); // Remove last character
                    }
                    KeyCode::Char(c) => {
                        input.push(c); // Append character to input
                    }
                    _ => {}
                }
            }
            execute!(
                stdout,
                MoveToColumn(1),
                Print(" ".repeat(x as usize)), // Print spaces up to `x`
                MoveToColumn(1),               // Move to column `x` where new text can be written
                Print(&input)
            )
            .unwrap();
        }
    }
}

pub fn display_tasks(wh: (u16, u16), conn: &Connection) {
    let mut stdout = stdout();
    let x: u16 = (wh.0 / 3 * 2) + 1;

    let data2: HashMap<bool, Vec<String>> =
        database::load_tasks(conn).expect("Failed to load tasks");

    execute!(
        stdout,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(1, 1)
    )
    .unwrap();

    if let Some(tasks) = data2.get(&false) {
        for (i, task) in tasks.iter().enumerate() {
            let j: u16 = (i + 1) as u16;
            execute!(
                stdout,
                cursor::MoveTo(1, j),
                Print(format!(" [ ] {}", task))
            )
            .unwrap();
        }
    }
    if let Some(tasks) = data2.get(&true) {
        for (i, task) in tasks.iter().enumerate() {
            let j: u16 = (i + 1) as u16;
            execute!(
                stdout,
                cursor::MoveTo(x, j),
                Print(format!(" [X] {}", task))
            )
            .unwrap();
        }
    }
    border::draw_border(&mut stdout, wh.0, wh.1);
}

pub fn delete_task(y: u16, wh: (u16, u16), conn: &mut Connection, is_done: bool) {
    let index: usize = (y - 1) as usize;

    let data2: HashMap<bool, Vec<String>> =
        database::load_tasks(conn).expect("Failed to load tasks");

    if let Some(tasks) = data2.get(&is_done) {
        if index < tasks.len() {
            let task = tasks[index].clone();
            database::remove_task(conn, task.as_str()).expect("Failed to remove task");
            display_tasks(wh, conn);
        }
    }
}

pub fn complete_task(y: u16, wh: (u16, u16), conn: &mut Connection) {
    let index: usize = (y - 1) as usize;
    let data2: HashMap<bool, Vec<String>> =
        database::load_tasks(conn).expect("Failed to load tasks");

    if let Some(tasks) = data2.get(&false) {
        if index < tasks.len() {
            let task = tasks[index].clone();
            database::complete_task(conn, task.as_str()).unwrap();
        }
    }
    display_tasks(wh, conn);
}

pub fn uncomplete_task(y: u16, wh: (u16, u16), conn: &mut Connection) {
    let index: usize = (y - 1) as usize;

    let data2: HashMap<bool, Vec<String>> =
        database::load_tasks(conn).expect("Failed to load tasks");

    if let Some(tasks) = data2.get(&true) {
        if index < tasks.len() {
            let task = tasks[index].clone();
            database::uncomplete_task(conn, task.as_str()).expect("Cannot uncomplete task");
        }
    }
    display_tasks(wh, conn);
}
