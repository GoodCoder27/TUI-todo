use crossterm::{
    cursor, execute,
    style::{Color, Print, SetForegroundColor},
};

pub fn draw_border(stdout: &mut std::io::Stdout, width: u16, height: u16) {
    execute!(stdout, SetForegroundColor(Color::Yellow)).unwrap();

    let horizontal = "─"; // Unicode box-drawing horizontal line
    let vertical = "│"; // Unicode box-drawing vertical line
    let top_left = "╭"; // Unicode rounded top-left corner
    let top_right = "╮"; // Unicode rounded top-right corner
    let bottom_left = "╰"; // Unicode rounded bottom-left corner
    let bottom_right = "╯"; // Unicode rounded bottom-right corner
    let t_top = "┬"; // Unicode t shaped top corner
    let t_bottom = "┴"; // Unicode t shaped bottom corner
                        // borders
    let border_one: u16 = width / 3;
    let border_two: u16 = border_one * 2;

    let todo = "TODO";
    let done = "DONE";

    // middles inside the borders
    let middle_one: u16 = (border_two - todo.len() as u16) / 2;
    let middle_two: u16 = border_two + ((border_one - done.len() as u16) / 2);

    // Draw horizontal borders
    for x in 0..width {
        execute!(stdout, cursor::MoveTo(x, 0), Print(horizontal)).unwrap();
        execute!(stdout, cursor::MoveTo(x, height - 1), Print(horizontal)).unwrap();
    }

    // Draw vertical borders
    for y in 0..height {
        execute!(stdout, cursor::MoveTo(0, y), Print(vertical)).unwrap();
        execute!(stdout, cursor::MoveTo(width - 1, y), Print(vertical)).unwrap();
    }

    for y in 0..height {
        execute!(stdout, cursor::MoveTo(border_two, y), Print(vertical)).unwrap();
    }

    // Draw corners
    execute!(stdout, cursor::MoveTo(0, 0), Print(top_left)).unwrap();
    execute!(stdout, cursor::MoveTo(width - 1, 0), Print(top_right)).unwrap();
    execute!(stdout, cursor::MoveTo(0, height - 1), Print(bottom_left)).unwrap();
    execute!(
        stdout,
        cursor::MoveTo(width - 1, height - 1),
        Print(bottom_right)
    )
    .unwrap();
    // Draw t-shapes
    execute!(stdout, cursor::MoveTo(border_two, 0), Print(t_top)).unwrap();
    execute!(
        stdout,
        cursor::MoveTo(border_two, height - 1),
        Print(t_bottom)
    )
    .unwrap();

    // write text
    execute!(stdout, cursor::MoveTo(middle_one, 0), Print(todo)).unwrap();
    execute!(stdout, cursor::MoveTo(middle_two, 0), Print(done)).unwrap();
}
