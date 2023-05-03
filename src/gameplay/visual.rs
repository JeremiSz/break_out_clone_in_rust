use std::io;

const ICONS:[char;3] = [' ','■','@'];
const IMAGE_SIZE:usize = super::SCREEN_BUFFER_SIZE + super::COL;

pub fn init()->io::Stdout{
    let mut writer = io::stdout();
    //execute!(writer, terminal::EnterAlternateScreen).unwrap();
    //terminal::enable_raw_mode().unwrap();
    writer   
}

pub fn draw<W>(w:&mut W,visual:&[char;super::SCREEN_BUFFER_SIZE]) -> io::Result<()>where W : io::Write
{
    let mut image:[u8;IMAGE_SIZE] = [0;IMAGE_SIZE];
    for i in 0..super::ROW{
        for j in 0..super::COL{
            image[i*super::COL + j] = visual[i*super::COL + j] as u8;
        }
        image[i*super::ROW] = b'\n';
    }
    w.write(&image)?;
    w.flush()?;
    io::Result::Ok(())
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