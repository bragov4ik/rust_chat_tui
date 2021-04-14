use std::io::Write;
use std::io::stdout;
use std::fmt;
use std::str::FromStr;
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

pub struct Message {
    timestamp: Timestamp,
    author: String,
    contents: String,
}

impl Message {
    pub fn from_raw(time: String, author: String, contents: String) -> Message{
        Message {
            timestamp: Timestamp{time},
            author,
            contents,
        }
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}:{}", self.timestamp, self.author, self.contents)
    }
}

/// Builds vector of strings with messages in reverse order
/// of arrival (newest - last) and in sequental order within
/// a message. Those lines do not exceed `max_line_len` by
/// 
/// Requires messages sorted in order of newest first
/// 
/// # Arguments
/// 
/// `messages: Vec<Message>` - list of messages sorted in newest first order
/// `max_line_len` - maximum result line length
pub fn build_messages_string_arr(messages: Vec<Message>, max_line_len: usize, max_lines: usize) -> Vec<String> {
    let mut result = vec!();
    for msg in messages {
        // Stop on exceeding terminal frame
        if result.len() >= max_lines {
            break;
        }

        let msg_string: String = format!("{}", msg);
        let mut msg_vector = vec!();

        // TODO proper length operation to work with multibyte
        // characters properly

        for i in (0..msg_string.len()).step_by(max_line_len) {
            let mut end_index = i+max_line_len;
            if end_index >= msg_string.len() {
                end_index = msg_string.len();
            }
            // This will likely to panic in case of multi-byte characters
            msg_vector.push(String::from_str(&msg_string[i..end_index]).unwrap());
        }
        msg_vector.append(&mut result);
        result = msg_vector;
    }
    result
}

/// Writes messages within specified frame
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
        // draw_window();
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