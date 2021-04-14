mod chat_tui;

use std::io;
use std::io::stdout;
use std::str::FromStr;
use crossterm::execute;

use chat_tui::{move_cursor_to_input_field, draw_window_contin};


// fn main() {
//     // std::thread::spawn(move || {
//     //     print_events().unwrap();
//     // });

//     execute!(stdout(), crossterm::terminal::EnterAlternateScreen);

//     move_cursor_to_input_field();
//     std::thread::spawn(move || {
//         draw_window_contin();
//     });

//     let mut buf = String::new();
//     io::stdin().read_line(&mut buf).unwrap();

//     println!("Read {}", &buf);
//     move_cursor_to_input_field();

//     let mut buf = String::new();
//     io::stdin().read_line(&mut buf).unwrap();

//     println!("Read {} \nType anything to leave:", &buf);
//     move_cursor_to_input_field();

//     let mut buf = String::new();
//     io::stdin().read_line(&mut buf).unwrap();
//     execute!(stdout(), crossterm::terminal::LeaveAlternateScreen);
    
//     // thread::park();
// }

fn main() {
    let test_vec = vec!(
        chat_tui::Message::from_raw(
            String::from_str("123").unwrap(), 
            String::from_str("aboba").unwrap(), 
            String::from_str("ABOBA").unwrap(),
        ),
        chat_tui::Message::from_raw(
            String::from_str("122").unwrap(), 
            String::from_str("cock").unwrap(), 
            String::from_str("cam").unwrap(),
        ),
        chat_tui::Message::from_raw(
            String::from_str("32").unwrap(), 
            String::from_str("cockerel").unwrap(), 
            String::from_str("beef").unwrap(),
        ),
    );
    let result = chat_tui::build_messages_string_arr(test_vec, 5, 8);
    for res in result {
        println!("{}", res);
    }
}