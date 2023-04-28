use std::io;
use std::thread;

use crossterm::{
    cursor,
    execute, queue, style,
    terminal
};

use super::GameState;
const ICONS :[char;5] = ['□','■','=','@',' '];
use std::sync::Arc;

pub fn set_up_terminal(game_state:Arc<GameState>) -> thread::JoinHandle<()>
{
    let mut writer = io::stdout();
    execute!(writer, terminal::EnterAlternateScreen).unwrap();
    terminal::enable_raw_mode().unwrap();
    thread::spawn(move || {rendering_loop(game_state,&mut writer)})
}
fn rendering_loop(game_state:Arc<GameState>,writer:&mut io::Stdout){
    let mut visual:[char;super::MAX + 2 * super::COL] = [' ';super::MAX + 2 * super::COL];
    loop{
        if game_state.game_ended{
            break;
        }
        render(&*game_state,&mut visual);
        draw(writer,&visual).unwrap();
    }
    execute!(writer,terminal::LeaveAlternateScreen).unwrap();
    terminal::disable_raw_mode().unwrap();
}
pub fn render(board:&GameState,visual:&mut[char;super::MAX + 2 * super::COL]){
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

pub fn draw<W>(w:&mut W,visual:&[char;super::MAX + 2 * super::COL]) -> io::Result<()>where W : io::Write
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