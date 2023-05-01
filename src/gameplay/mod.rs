use std::sync::mpsc;
use super::{Message,MessageCodes};

struct GameState {
    paddle_pos:i64,
    paddle_size:i64,
    paddle_vel:i64,
    block_poses:u64,
    ball_pos_x:i64,
    ball_pos_y:i64,
    ball_vel_x:i64,
    ball_vel_y:i64
}

pub fn start(visual_in:mpsc::Receiver<Message>,visual_out:mpsc::Sender<Message>){
    let mut game_state = GameState{
        paddle_pos:5,
        paddle_size:2,
        paddle_vel:0,
        block_poses:super::MAX,
        ball_pos_x:6,
        ball_pos_y:11,
        ball_vel_x:0,
        ball_vel_y:1
    };
    loop{
        let message = visual_in.try_recv();
        if message.is_ok(){
            let message = message.unwrap();
            match message.kind{
                MessageCodes::Exit => {break;},
                MessageCodes::MovePaddle => {
                    game_state.paddle_pos = message.data;
                    game_state.paddle_vel = message.data;
                }
                _ => {}
            }
        }
        handle_physics(&mut game_state,&visual_out);
        
    }
}
fn handle_physics(game_state:&mut GameState,visual_out:&mpsc::Sender<Message>){
    let mut new_x = game_state.ball_pos_x + game_state.ball_vel_x;
    let mut  new_y = game_state.ball_pos_y + game_state.ball_vel_y;

    let index = (new_y * (super::COL as i64) + new_x) as i8;
    let block_index = (1 as u64) << index;
    if game_state.block_poses & block_index != 0{
        game_state.block_poses &= !block_index;
        game_state.ball_vel_y *= -1;
        let data = game_state.block_poses  as i64;
        visual_out.send(Message{
            kind:MessageCodes::BlockChanged,
            data:data
        }).unwrap();
    }

    if new_x < 0{
        new_x = 0;
        game_state.ball_vel_x = 1;
    }
    else if new_x >= super::COL{
        new_x = super::COL - 1;
        game_state.ball_vel_x = -1;
    }

    if new_y < 0{
        new_y = 0;
        game_state.ball_vel_y = 1;
    }
    else if new_y >= super::ROW{
        new_y = super::ROW - 2;
        game_state.ball_vel_y = -1;
    }
    visual_out.send(Message{
        kind:MessageCodes::BallMoved,
        data:new_y * super::COL + new_x
    }).unwrap();
    game_state.ball_pos_x = new_x;
    game_state.ball_pos_y = new_y;
    println!("test {} {}",new_x,new_y);
}