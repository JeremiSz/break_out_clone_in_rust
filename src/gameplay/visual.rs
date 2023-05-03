use crossterm::{
    execute,
    terminal,
    queue,
    style,
    cursor
};

use std::io;

const ICONS:[char;3] = [' ','■','@'];

pub fn init()->io::Stdout{
    let mut writer = io::stdout();
    println!("made writer");
    let result = execute!(writer, terminal::EnterAlternateScreen);
    println!("{:?}",result);
    println!("changed to alternate screen");
    terminal::enable_raw_mode().unwrap();
    println!("enabled raw mode");
    writer   
}

pub fn draw<W>(w:&mut W,visual:[char;super::SCREEN_BUFFER_SIZE]) -> io::Result<()>where W : io::Write
{
    queue!(
        w,
        style::ResetColor,
        terminal::Clear(terminal::ClearType::All),
        cursor::Hide,
        cursor::MoveTo(0,0)
    )?;
    for i in 0..super::ROW{
        let (start,end) = ((i*super::COL) as usize,((i+1)*super::COL) as usize);
        let line = &visual[start..end];
        let string :String = line.iter().cloned().collect();
        println!("{}",string);
        queue!(
            w,
            style::Print(string),
            cursor::MoveToNextLine(1)
        )?;
    }
    w.flush()?;
    io::Result::Ok(())
}

pub fn conclude(mut writer: io::Stdout){
    execute!(writer,terminal::LeaveAlternateScreen).unwrap();
    terminal::disable_raw_mode().unwrap(); 
}

pub fn fill(spot :usize,visual :&mut [char;super::SCREEN_BUFFER_SIZE]){
    visual[spot] = ICONS[1];
}
pub fn empty(spot :usize,visual: &mut [char;super::SCREEN_BUFFER_SIZE]){
    visual[spot] = ICONS[0];
}
pub fn pellet(spot :usize,visual: &mut [char;super::SCREEN_BUFFER_SIZE]){
    visual[spot] = ICONS[2];
}