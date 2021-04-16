mod chat_tui;

use std::sync::{Arc, Mutex};
use std::str::FromStr;
use rand::seq::IteratorRandom;

fn one_user_chat() {
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

type Shared<T> = Arc<Mutex<T>>;

fn add_random_msg(msgs: &mut Shared<Vec<(String, String, String)>>, message_content: &mut Shared<String>) {
    let time_now = format!("{}", chrono::offset::Local::now().format("%H:%M:%S"));
    let nicknames = ["admin", "boss", "bidlo", "horek"];
    let mut rng = rand::thread_rng();
    let nick = nicknames.iter().choose(&mut rng).unwrap();
    let nick = String::from_str(nick).unwrap();
    let msg_content = String::from_str(&*(message_content.lock().unwrap())).unwrap();
    let msg = (time_now, nick, msg_content);
    msgs.lock().unwrap().insert(0, msg);
}

fn construct_messages(messages: Vec<(String, String, String)>) -> Vec<chat_tui::Message> {
    let mut result = vec!();
    for msg in messages {
        result.push(
            chat_tui::Message::from_strings(
                msg.0, 
                msg.1, 
                msg.2, 
            )
        );
    }
    result
}

fn write_random_msgs_every_sec(mut message_content: Shared<String>) {
    let mut messages: Shared<Vec<(String, String, String)>> = 
    Arc::new(Mutex::new(
        vec!(
            (
                String::from_str("16:31:41").unwrap(), 
                String::from_str("system").unwrap(), 
                String::from_str("ABOBA").unwrap(),
            ),
            (
                String::from_str("16:31:43").unwrap(), 
                String::from_str("system").unwrap(), 
                String::from_str("cam").unwrap(),
            ),
            (
                String::from_str("16:31:44").unwrap(), 
                String::from_str("system").unwrap(), 
                String::from_str("beef").unwrap(),
            ),
        )
    ));
    loop {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        add_random_msg(&mut messages, &mut message_content);
        chat_tui::draw_window(construct_messages(messages.lock().unwrap().clone()));
    }
}

fn multi_user_sim() {
    chat_tui::open_window(); 
    chat_tui::draw_window(vec!(""));

    let message_content = Arc::new(Mutex::new(String::new()));

    {
        let message_content = message_content.clone();
        std::thread::spawn(move || {
            write_random_msgs_every_sec(message_content)
        });
    }
    
    let mut buf = String::new();

    loop {
        // Read message
        chat_tui::read_input_line(&mut buf).unwrap();
        if buf.starts_with("/exit") {
            break
        }
        buf = buf.trim().to_string();
        message_content.lock().unwrap().push_str(&buf[..]);
        buf.clear();
    }

    chat_tui::close_window();
}

fn main() {
    multi_user_sim()
}