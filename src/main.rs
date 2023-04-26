use std::io;
pub use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue, style,
    terminal::{self, ClearType},
    Command,
};

const COL:usize = 10;
const ROW:usize = 10;
const MAX :usize = COL * ROW;

fn main() -> io::Result<()> {
    //let mut board:[usize;MAX] = [0;MAX];
    let mut visual:[char;MAX] = ['□';MAX];
    
    //let cursor = 44;
    
    let mut writer = set_up_terminal();    
    draw(&mut writer,&visual)?;
    io::Result::Ok(())
}

fn set_up_terminal()-> io::Stdout
{
    let mut writer = io::stdout();
    execute!(writer, terminal::EnterAlternateScreen).unwrap();
    terminal::enable_raw_mode().unwrap();
    writer
}
//fn input(){}
//fn update(){}
//fn render(){}
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
