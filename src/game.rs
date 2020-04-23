use crate::pieces;
use crate::board;
use regex::Regex;
use dialoguer::Input;

pub fn parse_command(s: &str) -> Option<(pieces::Pos, pieces::Pos)> {
    let re = Regex::new(r"^[a-h][0-8] [a-h][0-8]$").unwrap();
    if re.is_match(&s) {
        let src = &s[..2];
        let dst = &s[3..];
        let src_y = &src[..1].as_bytes()[0]-97;
        let dst_y = &dst[..1].as_bytes()[0]-97;
        let src_x = &src[1..].as_bytes()[0]-49;
        let dst_x = &dst[1..].as_bytes()[0]-49;

        Some((pieces::Pos(7-src_x, src_y), pieces::Pos(7-dst_x, dst_y)))
    } else {
        None
    }
}

pub fn take_input(turn: &str) -> String {
    let prompt = format!("({})", turn);
    let inp = Input::<String>::new().with_prompt(prompt.as_str()).interact().unwrap();
    inp
}

pub fn play<T>(mut my_board: board::Board<T>) 
    where T: pieces::Piece
{
    my_board.update_board();
    my_board.show();
    let mut turn = "white";
    loop {
        let command = take_input(turn);
        if command == String::from("quit") {
            break;
        }
        let command = parse_command(&command);
        let (mut src, mut dst): (pieces::Pos, pieces::Pos);
        match command {
            Some((src,dst)) => {
                if my_board.move_piece(src, dst, turn){
                    my_board.update_board();
                    my_board.show();
                    if turn == "white"{
                        turn = "black";
                    }else{
                        turn = "white";
                    }
                }
            },
            None => println!("bad instruction")
        };
    };
}


