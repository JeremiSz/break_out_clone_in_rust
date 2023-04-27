use std::io;
pub use crossterm::{
    cursor,
    execute, queue, style,
    terminal::{self, ClearType},
    Command,
};

pub const COL:usize = 10;
pub const ROW:usize = 10;
pub const MAX :usize = COL * ROW;
mod input;

fn main() -> io::Result<()> {
    let mut board:[usize;MAX] = [0;MAX];
    let mut visual:[char;MAX] = [' ';MAX];
    
    let mut cursor = 44;
    
    let mut writer = set_up_terminal(); 
    loop{   
        cursor = input::input(cursor);
        if cursor > MAX{
            break ;
        }
        render(&board, &mut visual,cursor);
        draw(&mut writer,&visual)?;
    }
    tear_down_terminal(&mut writer);
    io::Result::Ok(())
}

fn set_up_terminal()-> io::Stdout
{
    let mut writer = io::stdout();
    execute!(writer, terminal::EnterAlternateScreen).unwrap();
    terminal::enable_raw_mode().unwrap();
    writer
}
fn tear_down_terminal(w : &mut io::Stdout){
    execute!(w,terminal::LeaveAlternateScreen).unwrap();
    terminal::disable_raw_mode().unwrap();
}


//fn update(){}
fn render(board:&[usize;MAX],visual:&mut[char;MAX],selected:usize){
    for i in 0..MAX{
        let index:usize = if i == selected {board[i] + 1} else {board[i]};
        visual[i] = match index{
            0 => '□',
            1 => '■',
            _ => ' ',
        };
    }
}

fn draw<W>(w:&mut W,visual:&[char;MAX]) -> io::Result<()>
where W : io::Write
{
    queue!(
        w,
        style::ResetColor,
        terminal::Clear(terminal::ClearType::All),
        cursor::Hide,
        cursor::MoveTo(0,0)
    )?;
    for i in 0..ROW{
        let line = &visual[i*COL..(i+1)*COL];
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
