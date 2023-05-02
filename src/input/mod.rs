use std::sync::{Arc,Mutex};
use super::MessageCodes;
use crossterm::event::{self,Event,KeyEvent,KeyCode};

pub fn start(input:Arc<Mutex<MessageCodes>>){
    loop{
        let event = event::read().unwrap();
        let exit = match event{
            Event::Key(event) => 
                {handle_key(event,&input)},
            _ => {false}
        };
        if exit{
            break;
        }
    }
}

fn handle_key(
    event:KeyEvent,
    input:&Arc<Mutex<MessageCodes>>)
    -> bool
    {
    let keycode = event.code;
    let value = match keycode{
        KeyCode::Right => {MessageCodes::Right},
        KeyCode::Left => {MessageCodes::Left},
        KeyCode::Up => {MessageCodes::Up},
        KeyCode::Down => {MessageCodes::Down},
        KeyCode::Char('c') => {MessageCodes::Exit},
        _ => {MessageCodes::None}
    };
    if value != MessageCodes::None{
        input.lock().unwrap().set(value);
    }  
    value == MessageCodes::Exit 
}
