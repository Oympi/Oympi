use std::vec;

mod message;
mod security;


fn main() {
    println!("Hello, world!");
    message::Message::new("test".to_string(), [8].to_vec());
}
