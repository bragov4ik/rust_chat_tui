use std::io::Write;
use std::io::stdout;
use crossterm::cursor::{
    MoveToPreviousLine, 
    MoveToNextLine, 
    MoveToColumn, 
    SavePosition, 
    RestorePosition
};
use crossterm::execute;
use crossterm::queue;
use crossterm::style::Print;

fn queue_up_home() {
    queue!(
        stdout(),
        MoveToPreviousLine(1),
        MoveToColumn(0),
    );
}

fn queue_delimiter_line(columns: usize) {
    queue!(
        stdout(), 
        Print("-".repeat(columns)),
    );
}

fn queue_empty_line(columns: u16) {
    queue!(
        stdout(),
        MoveToColumn(columns),
    ); 
}

pub fn draw_window() {

    let (columns_u16, rows_u16) = crossterm::terminal::size().unwrap();
    queue!(
        stdout(), 
        crossterm::terminal::SetSize(columns_u16, rows_u16)
    );
    let columns: usize = columns_u16.into();
    queue!(
        stdout(), 
        SavePosition,
        MoveToNextLine(1),
        MoveToColumn(0),
    );
    queue_delimiter_line(columns);
    queue_up_home();
    queue_empty_line(columns_u16);
    queue_up_home();
    queue_delimiter_line(columns);
    queue_up_home();
    for i in 1..rows_u16-3 {
        queue_empty_line(columns_u16);
        queue_up_home();
    }
    queue_delimiter_line(columns);
    queue_up_home();

    queue!(
        stdout(),
        RestorePosition
    );
    stdout().flush().unwrap();
}

pub fn draw_window_contin() {
    loop {
        draw_window();
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}

pub fn move_cursor_to_input_field() {
    let rows_u16 = crossterm::terminal::size().unwrap().1;
    execute!(
        stdout(), 
        crossterm::cursor::MoveTo(0, rows_u16-2)
    );
}