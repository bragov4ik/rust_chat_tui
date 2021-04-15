mod chat_tui;

use std::str::FromStr;

fn main() {
    chat_tui::open_window();    
    let mut test_vec = vec!(
        chat_tui::Message::from_strings(
            String::from_str("123").unwrap(), 
            String::from_str("aboba").unwrap(), 
            String::from_str("ABOBA").unwrap(),
        ),
        chat_tui::Message::from_strings(
            String::from_str("122").unwrap(), 
            String::from_str("cock").unwrap(), 
            String::from_str("cam").unwrap(),
        ),
        chat_tui::Message::from_strings(
            String::from_str("32").unwrap(), 
            String::from_str("cockerel").unwrap(), 
            String::from_str("beef").unwrap(),
        ),
    );

    chat_tui::draw_window(test_vec.clone());

    let mut buf = String::new();

    loop {
        // Read message
        chat_tui::read_input_line(&mut buf).unwrap();
        if buf.starts_with("/exit") {
            break
        }
        // Build message for output
        buf = buf.trim().to_string();
        let time_now = format!("{}", chrono::offset::Local::now().format("%H:%M:%S"));
        test_vec.insert(
            0,
            chat_tui::Message::from_strings(time_now, String::from_str("system").unwrap(), buf.clone()),
        );
        buf.clear();

        // Print messages
        chat_tui::draw_window(test_vec.clone());
    }

    chat_tui::close_window();
}