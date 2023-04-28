pub const COL:usize = 11;
pub const ROW:usize = 11;
pub const MAX :usize = COL * ROW;
mod input;
mod visuals;
struct GameState {
    paddle_pos:usize,
    paddle_size:usize,
    block_poses:u128,
    ball_pos_x:usize,
    ball_pos_y:usize,
    ball_vel_x:i64,
    ball_vel_y:i64,
    game_ended:bool
}
use std::sync::Arc;

fn main() {
    let mut board:GameState = GameState{
        paddle_pos:6,
        paddle_size:1,
        block_poses:(MAX as u128),
        ball_pos_x:6,
        ball_pos_y:0,
        ball_vel_x:0,
        ball_vel_y:1,
        game_ended:false
    };
    let board_ref = Arc::new(board);
    let rendering_thread_handler = visuals::set_up_terminal(board_ref); 
    
    loop{   
        input::input(&mut board);
        if  (*board_ref).game_ended{
            rendering_thread_handler.join().unwrap();
            break ;
        }
    }
}
