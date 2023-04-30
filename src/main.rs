use std::thread;
use std::sync::mpsc;
mod visuals;

pub struct Vec2{
    x:i64,
    y:i64
}
pub struct Message{
    kind:MessageCodes,
    data:i64
}
pub enum MessageCodes {
    Exit = 0
}

pub const COL:usize = 11;
pub const ROW:usize = 11;
pub const MAX :usize = COL * ROW;
fn main() {
    let (input_visual_sender, input_visual_reciever):(mpsc::Sender<Message>,mpsc::Receiver<Message>) = mpsc::channel();
    let (visual_gameplay_sender, visual_gameplay_reciever):(mpsc::Sender<Message>,mpsc::Receiver<Message>)  = mpsc::channel();
    let (gameplay_visual_sender, gameplay_visual_reciever):(mpsc::Sender<Message>,mpsc::Receiver<Message>)  = mpsc::channel();
    let visual_handle = thread::spawn(move || {
        visuals::start(input_visual_reciever,visual_gameplay_sender,gameplay_visual_reciever)
    });

    
}
