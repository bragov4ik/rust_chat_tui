use std::io::Write;
use std::io::stdout;
use std::fmt;
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

struct Timestamp {
    time: String
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.time)
    }
}

struct Message {
    timestamp: Timestamp,
    author: String,
    contents: String,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}:{}", self.timestamp, self.author, self.contents)
    }
}

/// Builds vector of strings with
/// Requires messages sorted in order of newest first
fn build_messages_string_arr(messages: Vec<Message>, max_line_len: usize) -> Vec<String> {
    let result = vec!();
    for msg in messages {
        let msg_string: String = format!("{}", msg);
    }
    vec!(String::new())
}

fn add_messages() {

}

/// Adds commands to move one line up and go to the first column
/// in the queue.
fn add_up_home() {
    queue!(
        stdout(),
        MoveToPreviousLine(1),
        MoveToColumn(0),
    );
}

/// Adds command to write separating line for the whole row
/// in the queue. 
fn add_delimiter_line(columns: usize) {
    queue!(
        stdout(), 
        Print("-".repeat(columns)),
    );
}

/// Adds command to draw line for contents in the queue.
fn add_empty_line(columns: u16) {
    queue!(
        stdout(),
        MoveToColumn(columns),
    ); 
}


pub fn draw_window(messages: Vec<Message>) {

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
    add_delimiter_line(columns);
    add_up_home();
    add_empty_line(columns_u16);
    add_up_home();
    add_delimiter_line(columns);
    add_up_home();

    // Write messages
    for i in 1..rows_u16-3 {
        if i == 4 {
            queue!(
                stdout(), 
                Print("Multiline\ncursor\ntest"),
                Print("Können"),
            );
        }
        add_empty_line(columns_u16);
        add_up_home();
    }

    add_delimiter_line(columns);
    add_up_home();

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