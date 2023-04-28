pub use crossterm::{
    event::{self, Event, KeyCode, KeyEvent}
};

use super::GameState;
use std::cmp;

pub fn input(game_state : &mut GameState){
    let event = event::read().unwrap();
    match event {
        Event::Key(event) => {handle_key(event, game_state)}
        _ => {}
    }
}
fn handle_key(event:KeyEvent,game_state : &mut GameState){
    let keycode = event.code;
    match keycode{
        KeyCode::Right => {move_right(game_state)},
        KeyCode::Down => {move_right(game_state)},
        KeyCode::Char('c') => {game_state.game_ended = true;},
        _ => {}
    }
}
fn move_right(game_state : &mut GameState){
    let current_pos = game_state.paddle_pos;
    game_state.paddle_pos = cmp::min(super::COL-game_state.paddle_size,current_pos + 1);
}
fn move_left(game_state : &mut GameState){
    let current_pos = game_state.paddle_pos;
    game_state.paddle_pos = cmp::max(0,current_pos - 1);
}