// 1. create add tasks
// 2. remove tasks
// 3. complete tasks
// 4. currently doing tasks
// 5. group tasks
// 6. load tasks or new task
// 7. command line functionality

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{self, ClearType},
};
use rusqlite::Connection;
use std::io::stdout;
use tasks::display_tasks;

mod border;
mod database;
mod tasks;

const MIN_WIDTH: u16 = 84;
const MIN_HEIGHT: u16 = 17;

fn main() -> rusqlite::Result<()> {
    let mut stdout = stdout();
    let mut conn = Connection::open("tasks.db")?;
    database::initialize_database(&conn).expect("Failed to initialize database");

    // Enter raw mode
    terminal::enable_raw_mode().unwrap();

    loop {
        let (width, height) = terminal::size().unwrap();
        execute!(
            stdout,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )
        .unwrap();

        let mut x = 1;
        let mut y = 1;
        let mut is_done: bool = false;

        if width >= MIN_WIDTH && height >= MIN_HEIGHT {
            execute!(stdout, SetForegroundColor(Color::Yellow)).unwrap();
            display_tasks((width, height), &conn);
            // border::draw_border(&mut stdout, width, height);

            execute!(stdout, cursor::MoveTo(x, y)).unwrap();
        } else {
            execute!(stdout, SetForegroundColor(Color::Red)).unwrap();
            // Show warning if the screen is too small
            execute!(stdout, Print("Terminal too small!\n")).unwrap();
            execute!(
                stdout,
                Print(&format!("Needs at least {}x{}.\n", MIN_WIDTH, MIN_HEIGHT))
            )
            .unwrap();
            execute!(stdout, Print("Resize the window or press 'q' to exit.")).unwrap();
        }

        // Wait for key press or screen resize
        loop {
            if let Ok(true) = event::poll(std::time::Duration::from_millis(100)) {
                if let Ok(Event::Key(KeyEvent { code, .. })) = event::read() {
                    match code {
                        KeyCode::Char('q') => {
                            terminal::disable_raw_mode().unwrap();
                            execute!(stdout, terminal::Clear(ClearType::All)).unwrap();
                            return Ok(());
                        }
                        KeyCode::Char('k') if y > 1 => {
                            y -= 1;
                        }
                        KeyCode::Char('j') if y < height - 2 => {
                            y += 1;
                        }
                        KeyCode::Char('h') if is_done => {
                            x -= (width / 3) * 2;
                            is_done = false;
                        }
                        KeyCode::Char('l') if !is_done => {
                            x += (width / 3) * 2;
                            is_done = true;
                        }
                        KeyCode::Char('a') if !is_done => tasks::create_task(width, &mut conn),
                        KeyCode::Char('r') => {
                            tasks::delete_task(y, (width, height), &mut conn, is_done)
                        }
                        KeyCode::Enter => {
                            if is_done {
                                tasks::uncomplete_task(y, (width, height), &mut conn);
                            }
                            if !is_done {
                                tasks::complete_task(y, (width, height), &mut conn);
                            }
                        }
                        _ => {}
                    }
                    execute!(stdout, cursor::MoveTo(x, y)).unwrap();
                }
            }

            // Detect if terminal size has changed
            let (new_width, new_height) = terminal::size().unwrap();
            if new_width != width || new_height != height {
                break; // Redraw everything if resized
            }
        }
    }
}
