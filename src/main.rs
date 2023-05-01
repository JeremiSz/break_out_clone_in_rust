use std::thread;
use std::sync::mpsc;
mod visuals;
mod input;
mod gameplay;

pub struct Message{
    kind:MessageCodes,
    data:i64
}
#[derive(PartialEq,Copy,Clone)]
pub enum MessageCodes {
    None,
    Exit,
    MovePaddle,
    BlockChanged,
    BallMoved
}

pub const COL:i64 = 8;
pub const ROW:i64 = 8;
pub const MAX :u64 = (COL as u64) * (ROW as u64);
fn main() {
    let (input_visual_sender, input_visual_reciever):(mpsc::Sender<Message>,mpsc::Receiver<Message>) = mpsc::channel();
    let (visual_gameplay_sender, visual_gameplay_reciever):(mpsc::Sender<Message>,mpsc::Receiver<Message>)  = mpsc::channel();
    let (gameplay_visual_sender, gameplay_visual_reciever):(mpsc::Sender<Message>,mpsc::Receiver<Message>)  = mpsc::channel();
    let visual_handle = thread::spawn(move || {
        visuals::start(input_visual_reciever,visual_gameplay_sender,gameplay_visual_reciever)
    });
    let input_handle = thread::spawn(move ||{
        input::start(input_visual_sender);
    });
    let gameplay_handle = thread::spawn(move ||{
        gameplay::start(visual_gameplay_reciever,gameplay_visual_sender)
    });

    input_handle.join().unwrap();
    visual_handle.join().unwrap();
    gameplay_handle.join().unwrap();
}
