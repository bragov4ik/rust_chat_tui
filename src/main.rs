mod chat_tui;

use std::io;
use std::io::stdout;
use crossterm::execute;

use chat_tui::{move_cursor_to_input_field, draw_window_contin};


fn main() {
    // std::thread::spawn(move || {
    //     print_events().unwrap();
    // });

    execute!(stdout(), crossterm::terminal::EnterAlternateScreen);

    move_cursor_to_input_field();
    std::thread::spawn(move || {
        draw_window_contin();
    });

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    println!("Read {}", &buf);
    move_cursor_to_input_field();

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    println!("Read {} \nType anything to leave:", &buf);
    move_cursor_to_input_field();

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    execute!(stdout(), crossterm::terminal::LeaveAlternateScreen);
    
    // thread::park();
}