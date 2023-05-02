use std::sync::{Arc,Mutex};
use std::time::{SystemTime,UNIX_EPOCH};
use super::MessageCodes;

pub fn start(input : Arc<Mutex<MessageCodes>>,col:usize,row:usize){
    let mut  snake_index:Vec<usize> = Vec::with_capacity(col * row);
    snake_index.push(0);
    let mut visual :[char;col*row]=[' ';col*row];
    let mut pellet = (col*row)/2;
    loop{
        let direction = get_input(&input);
        if direction == MessageCodes::Exit{
            break;
        }
        if direction != MessageCodes::None{
            move_snake(&snake_index,direction,col,row);
        }
        pellet = grow(&mut snake_index,pellet,col*row);
        if detect_collision(&snake_index){
            break;
        }
        
    }
}
fn get_input(input :&Arc<Mutex<MessageCodes>>)->MessageCodes{
    *input.lock().unwrap()
}
fn move_snake(snake:&Vec<usize>,direction:MessageCodes,col:usize,size:usize){
    let mut new_pos = match direction{
        MessageCodes::Up => {snake[0] - col},
        MessageCodes::Down => {snake[0] + col},
        MessageCodes::Left => {snake[0] - 1},
        MessageCodes::Right => {snake[0] + 1},
        _ => {snake[0]}
    };
    let mut old_pos;
    for i in 0..snake.len(){
        old_pos = snake[i];
        snake[i] = new_pos;
        new_pos = old_pos;
    };
}

fn detect_collision(snake:&Vec<usize>)->bool{
    let target = snake[0];
    let i = 1;
    while i < snake.len() && snake[i] != target{
        i += 1;
    }
    snake[i-1] == target
}
fn grow(snake:&mut Vec<usize>,pellet:usize,size:usize)->usize{
    if snake[0] == pellet{
        snake.push(0);
        random()%size
    }
    else{
        pellet
    }
}
fn random()->usize{
    SystemTime::now().duration_since(UNIX_EPOCH).expect("").as_secs() as usize
}