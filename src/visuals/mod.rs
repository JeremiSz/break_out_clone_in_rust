use crossterm::{
    cursor,
    execute, queue, style,
    terminal
};
use std::sync::mpsc;
use std::io;
use std::cmp;
use super::Message;
use super::MessageCodes;

const ICONS :[char;5] = ['□','■','=','@',' '];
const SCREEN_BUFFER_SIZE:usize = (super::MAX + 2 * (super::COL as u64)) as usize;

pub struct GameState {
    paddle_pos:i64,
    paddle_size:i64,
    block_poses:u64,
    ball_index:u64,
    game_ended:bool
}

pub fn start(
    input_in:mpsc::Receiver<Message>,
    gameplay_out:mpsc::Sender<Message>,
    gameplay_in:mpsc::Receiver<Message>){
        let mut game_state = GameState{
            paddle_pos:5,
            paddle_size:1,
            block_poses:super::MAX,
            ball_index:6 + (super::COL as u64),
            game_ended:false
        };
        let mut writer = io::stdout();
        let mut visual:[char;SCREEN_BUFFER_SIZE] = [' ';SCREEN_BUFFER_SIZE];
        
        //set_up_terminal(&mut writer);
        loop{
            handle_messages(&input_in,&gameplay_in,&gameplay_out,&mut game_state);
            if game_state.game_ended{break;}

            render(&game_state,&mut visual);
            //draw(&mut writer,&visual).unwrap();
        }
        //tear_down_terminal(&mut writer);
}
fn handle_messages(
    input_in:&mpsc::Receiver<Message>,
    gameplay_in:&mpsc::Receiver<Message>,
    gameplay_out:&mpsc::Sender<Message>,
    game_state:&mut GameState){

    let input_result = input_in.try_recv();
    let gameplay_result = gameplay_in.try_recv();

    if input_result.is_ok(){
        handle_message(input_result.unwrap(), &gameplay_out,game_state);
    };
    if  gameplay_result.is_ok(){
        handle_message(gameplay_result.unwrap(), &gameplay_out,game_state);
    };
}
fn handle_message(message:Message,gameplay_out:&mpsc::Sender<Message>,game_state:&mut GameState){
    match message.kind{
        MessageCodes::Exit => {exit(&gameplay_out,game_state)},
        MessageCodes::MovePaddle => {move_paddle(message, gameplay_out, game_state)}
        _ => {}
    };
}
fn move_paddle(message:Message,gameplay_out:&mpsc::Sender<Message>,game_state:&mut GameState){
    let new_pos = game_state.paddle_pos as i64;
    let new_pos = new_pos + message.data;
    let new_pos = cmp::max(0,new_pos);
    game_state.paddle_pos = cmp::min(super::COL - game_state.paddle_size,new_pos);
    let result = gameplay_out.send(Message{
        kind:MessageCodes::BlockChanged,
        data:(game_state.paddle_pos-32) as i64
    });
    if result.is_err(){
        println!("{}",result.unwrap_err());
    }
}
fn exit(gameplay_out:&mpsc::Sender<Message>,game_state:&mut GameState){
    gameplay_out.send(Message{
        kind:MessageCodes::Exit,
        data:0
    }).unwrap();
    game_state.game_ended = true;
}

fn set_up_terminal(writer:&mut io::Stdout){
    execute!(writer, terminal::EnterAlternateScreen).unwrap();
    terminal::enable_raw_mode().unwrap();
}
fn tear_down_terminal(writer:&mut io::Stdout){
    execute!(writer,terminal::LeaveAlternateScreen).unwrap();
    terminal::disable_raw_mode().unwrap();
}

fn render(board:&GameState,visual:&mut [char;SCREEN_BUFFER_SIZE]){
    for i in 0..(super::MAX as usize){
        if board.block_poses & (1<<i) != 0{
            visual[i] = ICONS[1];
        }
        else {
            visual[i] = ICONS[0];
        }
    }
    visual[board.ball_index as usize] = ICONS[3];
    let bar_indeices = (board.paddle_pos as u64)..((board.paddle_size+board.paddle_pos) as u64);
    for i in 0..(super::COL as u64){
        let index = (super::MAX + i + (super::COL as u64)) as usize;
        visual[index] = if bar_indeices.contains(&i) {ICONS[2]} else {ICONS[4]};
    }
}

fn draw<W>(w:&mut W,visual:&[char;SCREEN_BUFFER_SIZE]) -> io::Result<()>where W : io::Write
{
    queue!(
        w,
        style::ResetColor,
        terminal::Clear(terminal::ClearType::All),
        cursor::Hide,
        cursor::MoveTo(0,0)
    )?;
    for i in 0..super::ROW + 2{
        let (start,end) = ((i*super::COL) as usize,((i+1)*super::COL) as usize);
        let line = &visual[start..end];
        let string :String = line.iter().cloned().collect();
        queue!(
            w,
            style::Print(string),
            cursor::MoveToNextLine(1)
        )?;
    }
    w.flush()?;
    io::Result::Ok(())
}