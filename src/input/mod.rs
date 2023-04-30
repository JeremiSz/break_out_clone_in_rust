use std::sync::mpsc;
use super::{Message,MessageCodes};
use crossterm::event::{self,Event,KeyEvent,KeyCode};

pub fn start(visual_out:mpsc::Sender<Message>){

    loop{
        let event = event::read().unwrap();
        let kind = match event {
            Event::Key(event) => {handle_key(event,&visual_out)}
            _ => {MessageCodes::None}
        };
        if kind == MessageCodes::Exit{
            break;
        }
    }
}

fn handle_key(event:KeyEvent,visual_out:&mpsc::Sender<Message>)->MessageCodes{
    let keycode = event.code;
    let result = match keycode{
        KeyCode::Right => {Option::Some(Message{kind:MessageCodes::MovePaddle,data:1})},
        KeyCode::Left => {Option::Some(Message{kind:MessageCodes::MovePaddle,data:-1})},
        KeyCode::Char('c') => {Option::Some(Message{kind:MessageCodes::Exit,data:0})},
        _ => {Option::None}
    };
    if let Some(message) = result{
        let kind = message.kind;
        visual_out.send(message).unwrap();
        return kind;
    }
    MessageCodes::None
}