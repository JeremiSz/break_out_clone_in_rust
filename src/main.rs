#[derive(PartialEq,Debug,Clone,Copy)]
pub enum MessageCodes{
    None,
    Exit,
    Up,
    Down,
    Left,
    Right
}
impl MessageCodes{
    fn set(&mut self,code:MessageCodes){
        *self = code;
    }
}

use std::sync::{Arc,Mutex};
use std::thread;
use std::env;

mod input;
mod gameplay;
fn main(){
    

    let message_ref = Arc::new(Mutex::new(MessageCodes::None));
    let input_handle;
    {
        let input_ref = message_ref.clone();
        input_handle = thread::spawn(move || {input::start(input_ref)});
    }
    let (col,row) = parse_args(env::args().collect());
    assert!((col * row) < (100*100 + 1));
    gameplay::start(message_ref,col,row);
    input_handle.join().unwrap();
}

fn parse_args(args:Vec<String>)->(usize,usize){
    let col = match args[1].parse::<usize>() {
        Ok(n) => {n},
        Err(_) => {8}
    };
    let row = match args[2].parse::<usize>() {
        Ok(n) => {n},
        Err(_) => {8}
    };
    (col,row)
}