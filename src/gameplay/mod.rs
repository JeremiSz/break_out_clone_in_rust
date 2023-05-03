use std::sync::{Arc,Mutex};
use std::time::{SystemTime,UNIX_EPOCH};
use super::MessageCodes;

mod visual;

const COL:usize = 100;
const ROW:usize = 100;
const SCREEN_BUFFER_SIZE:usize = COL * ROW;

pub fn start(input : Arc<Mutex<MessageCodes>>){
    let mut snake_index:Vec<usize> = Vec::with_capacity(COL * ROW);
    snake_index.push(0);
    let mut pellet = (SCREEN_BUFFER_SIZE)/2;
    let mut visual_buffer:[char;SCREEN_BUFFER_SIZE] = [' ';SCREEN_BUFFER_SIZE];
    visual::pellet(pellet,&mut visual_buffer);
    let mut writer = visual::init();
    println!("initalised visual");
    loop{
        println!("entered loop");
        let direction = get_input(&input);
        println!("got input");
        if direction == MessageCodes::Exit{
            break;
        }
        println!("checked break");
        if direction != MessageCodes::None{
            move_snake(&mut snake_index,direction,&mut visual_buffer);
        }
        println!("moved");
        pellet = grow(&mut snake_index,pellet,&mut visual_buffer);
        println!("grew");
        if detect_collision(&snake_index){
            break;
        }
        println!("{:?} test",&visual_buffer);
        visual::draw(&mut writer,visual_buffer).unwrap();
        println!("drawn");
    }
    visual::conclude(writer);
}
fn get_input(input :&Arc<Mutex<MessageCodes>>)->MessageCodes{
    *input.lock().unwrap()
}
fn move_snake(snake:&mut Vec<usize>,direction:MessageCodes,visual:&mut [char;SCREEN_BUFFER_SIZE]){
    let mut new_pos = match direction{
        MessageCodes::Up => {snake[0] - COL},
        MessageCodes::Down => {snake[0] + COL},
        MessageCodes::Left => {snake[0] - 1},
        MessageCodes::Right => {snake[0] + 1},
        _ => {snake[0]}
    };
    visual::fill(new_pos,visual);

    let mut old_pos;
    for i in 0..snake.len(){
        old_pos = snake[i];
        snake[i] = new_pos;
        new_pos = old_pos;
    };
    visual::empty(new_pos,visual); 
}

fn detect_collision(snake:&Vec<usize>)->bool{
    let target = snake[0];
    let mut i = 1;
    while i < snake.len() && snake[i] != target{
        i += 1;
    }
    snake[i-1] == target
}
fn grow(snake:&mut Vec<usize>,pellet:usize,visual:&mut [char;SCREEN_BUFFER_SIZE])->usize{
    if snake[0] == pellet{
        snake.push(0);
        let new_pellet = random()%SCREEN_BUFFER_SIZE;
        visual::pellet(new_pellet,visual);
        new_pellet
    }
    else{
        pellet
    }
}
fn random()->usize{
    SystemTime::now().duration_since(UNIX_EPOCH).expect("").as_secs() as usize
}