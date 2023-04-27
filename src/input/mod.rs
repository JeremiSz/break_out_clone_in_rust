pub use crossterm::{
    event::{self, Event, KeyCode, KeyEvent}
};
pub fn input(selected : usize) -> usize{
    let mut selected = selected;
    let event = event::read().unwrap();
    match event {
        Event::Key(event) => {selected = handle_key(event, selected)}
        _ => {}
    }
    selected
}
fn handle_key(event:KeyEvent,selected:usize)->usize{
    let keycode = event.code;
    match keycode{
        KeyCode::Up => {if selected > super::COL {selected - super::COL} else{selected} }
        KeyCode::Down => {if selected < (super::MAX - super::COL) {selected + super::COL} else {selected}}
        KeyCode::Char('c') => {super::MAX + 1}
        _ => {selected}
    }
}