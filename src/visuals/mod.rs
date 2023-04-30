use crossterm::{
    cursor,
    execute, queue, style,
    terminal
};
use std::sync::mpsc;
use std::io;
use super::Message;

const ICONS :[char;5] = ['□','■','=','@',' '];

pub struct GameState {
    paddle_pos:usize,
    paddle_size:usize,
    block_poses:u128,
    ball_pos_x:usize,
    ball_pos_y:usize
}

pub fn start(
    input_in:mpsc::Receiver<Message>,
    gameplay_out:mpsc::Sender<Message>,
    gameplay_in:mpsc::Receiver<Message>){
        let mut game_state = GameState{
            paddle_pos:6,
            paddle_size:1,
            block_poses:(super::MAX as u128),
            ball_pos_x:6,
            ball_pos_y:0
        };
        let mut writer = io::stdout();
        let mut visual:[char;super::MAX + 2 * super::COL] = [' ';super::MAX + 2 * super::COL];
        let mut game_ended:bool = false;
        
        set_up_terminal(writer);
        loop{
            if game_ended{
                gameplay_out.send(Message{
                    kind:MessageCodes::Exit,
                    data:0
                }).unwrap();
                break;
            }
            render(game_state,visual);
            draw(writer,visual).unwrap();
        }
        tear_down_terminal(writer);
}

fn set_up_terminal(writer:io::Stdout){
    execute!(writer, terminal::EnterAlternateScreen).unwrap();
    terminal::enable_raw_mode().unwrap();
}
fn tear_down_terminal(writer:io::Stdout){
    execute!(writer,terminal::LeaveAlternateScreen).unwrap();
    terminal::disable_raw_mode().unwrap();
}

fn render(board:&GameState,visual:&mut [char;super::MAX + 2 * super::COL]){
    for i in 0..super::MAX{
        if board.block_poses & (1<<i) != 0{
            visual[i] = ICONS[1];
        }
        else {
            visual[i] = ICONS[0];
        }
        if i == board.ball_pos_y * super::COL + board.ball_pos_x{
            visual[i] = ICONS[3];
        }
    }
    for i in board.paddle_pos..board.paddle_size+board.paddle_pos{
        visual[super::MAX + super::COL + i] = ICONS[2];
    }
}

fn draw<W>(w:&mut W,visual:&[char;super::MAX + 2 * super::COL]) -> io::Result<()>where W : io::Write
{
    queue!(
        w,
        style::ResetColor,
        terminal::Clear(terminal::ClearType::All),
        cursor::Hide,
        cursor::MoveTo(0,0)
    )?;
    for i in 0..super::ROW{
        let line = &visual[i*super::COL..(i+1)*super::COL];
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