use std::io;
use std::io::BufWriter;
use std::io::Write;

fn main() {
    let mut board:[usize;100] = [0;100];
    board[44] = 1;
    render(&board);
}

fn render(board:&[usize;100]){
    let mut writer = BufWriter::new(io::stdout());
    let characters = ["□", "■"];
    for i in 0..board.len(){
        let _result = writer.write(characters[board[i]].as_bytes());
        if (i % 10) == 9{
            let _result = writer.write("\n".as_bytes());
        }
    }
    let _size = writer.flush();
}
