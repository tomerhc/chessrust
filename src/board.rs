use std::collections::HashMap;
use std::cmp;
use std::convert::TryInto;
use crate::pieces;
use crate::pieces::Piece;


pub struct Board 
{
    pub board: Vec<Vec<Option<Box<dyn Piece>>>>,
}

impl Board
{
    pub fn new() -> Board{
        let mut board = Vec::with_capacity(8);
        //let eaten_white = vec![];
        //let eaten_black = vec![];

        for _x in 0..8 {
            let mut row = Vec::with_capacity(8);
            for _y in 0..8 {
                row.push(None);
            }
            board.push(row);
        }

        Board {
            board: board,
            //eaten_white: eaten_white,
            //eaten_black: eaten_black
        }
    }

    pub fn put_piece (&mut self, piece: Box<dyn Piece>) {
        let pieces::Pos(x,y) = piece.get_cur_pos();
        self.board[x as usize][y as usize] = Some(piece);
    }
    
//    pub fn eat_piece (&mut self, pos: pieces::Pos){
//        let pieces::Pos(x,y) = pos;
//        let p = self.board[x as usize].remove(y as usize).unwrap();
//        self.board[x as usize].insert(y as usize, None);
//        match p.get_color() {
//            true => self.eaten_white.push(p),
//            false => self.eaten_black.push(p)
//        };
//    }
//
    pub fn show(&self) {
        println!("    a   b   c   d   e   f   g   h  ", );
        println!("  +---+---+---+---+---+---+---+---+", );
        let mut display_vec = vec![];
        for (index,row) in self.board.iter().enumerate() {
            for square in row.iter(){
                match square {
                    Some(piece) => display_vec.push(format!(" {} ",piece.show())),
                    None => display_vec.push(String::from("   "))
                }
            }
            println!("{} |{}|{}|{}|{}|{}|{}|{}|{}| {}",
                8 - index,
                display_vec[0],
                display_vec[1],
                display_vec[2],
                display_vec[3],
                display_vec[4],
                display_vec[5],
                display_vec[6],
                display_vec[7],
                8-index,
                );
            println!("  +---+---+---+---+---+---+---+---+", );
            display_vec.clear();
        }
        println!("    a   b   c   d   e   f   g   h  ", );
    }

    pub fn check_turn(&self, pos_from: &pieces::Pos, pos_to: &pieces::Pos, turn: &str) -> bool {
        let turn_bool = turn == "white";
        let mut res = true;
        let pieces::Pos(x,y) = pos_from;
        let piece = &self.board[*x as usize][*y as usize];
        match piece {
            Some(p) => {
                if p.get_color() != turn_bool{
                    println!("wrong piece, it's {}'s turn_bool", turn);
                    res = false;
                }
            },
            None => {
                println!("no piece in this location");
                res =  false;
            }
        }

        let pieces::Pos(x,y) = pos_to;
        let piece = &self.board[*x as usize][*y as usize];
        match piece {
            Some(p) => {
                if p.get_color() == turn_bool {
                    println!("cant eat your own piece");
                    res = false;
                }   
            },
            None => ()
        }
        res
    }

    pub fn move_piece(&mut self, pos_from: pieces::Pos, pos_to: pieces::Pos, turn: &str) -> bool {
    if self.check_turn(&pos_from, &pos_to, turn) {
        let pieces::Pos(x,y) = pos_from;
        let state = self.get_state(pos_from).clone();
        let piece = &mut self.board[x as usize][y as usize];
        match piece {
            Some(p) => {
                if p.move_piece(state, pos_to){
                    return true;
                }else{
                    return false;
                }
            },
            None => panic!("check turn came ot positive but no piece in location")
            }
        }else{
        return false;
        }
    }

    pub fn check_cut_state(&self, pos: &pieces::Pos ,v: Vec<pieces::Pos>) -> Vec<pieces::Pos>{
        let mut cur_square = 0 as usize;
        for (index, p) in v.iter().enumerate(){
            if p == pos {
                cur_square = index;
                break;
            }
        }

        let mut left = v;
        let mut right = left.split_off(cur_square);
        left.reverse();
        right.remove(0);
        let mut left_full_index = left.len()+1 as usize;
        let mut right_full_index = right.len()+1 as usize;

        
        for (index, p) in left.iter().enumerate(){
            let pieces::Pos(x,y) = p;
            match self.board[*x as usize][*y as usize] {
                Some(_) => {left_full_index = index; break;},
                None => ()
            }
        }

        for (index, p) in right.iter().enumerate(){
            let pieces::Pos(x,y) = p;
            match self.board[*x as usize][*y as usize] {
                Some(_) => {right_full_index = index; break;},
                None => ()
            }
        }
        
        left.truncate(left_full_index);
        right.truncate(right_full_index);
        left.reverse();
        left.append(&mut right);
        left
    }

    pub fn get_state(&self, pos: pieces::Pos) -> HashMap<String, Vec<pieces::Pos>> {
        let pieces::Pos(x,y) = pos;
        let mut col = Vec::with_capacity(8);
        let mut row = Vec::with_capacity(8);
        let mut diag1 = Vec::with_capacity(8);
        let mut diag2 = Vec::with_capacity(8);

        for i in 0..8{
            row.push(pieces::Pos(x,i));
            col.push(pieces::Pos(i,y));
        }
        
        let xi8 = x as i8;
        let yi8 = y as i8;

        let d1_low = cmp::max(xi8-8, -yi8-1);
        let d1_high = cmp::min(xi8+1, 8-yi8);
        for i in d1_low+1..d1_high {
            
            diag1.push(pieces::Pos((xi8-i) as u8, (yi8+i) as u8));
        }
        diag1.reverse(); //to make it the same direction as diag2

        let d2_low = cmp::max(-xi8-1, -yi8-1);
        let d2_high = cmp::min(8-yi8, 8-xi8);
        for i in d2_low+1..d2_high {
            diag2.push(pieces::Pos((xi8+i) as u8, (yi8+i) as u8));
        }
        
        let mut res = HashMap::new();
        res.insert(String::from("row"), self.check_cut_state(&pos, row));
        res.insert(String::from("col"), self.check_cut_state(&pos, col));
        res.insert(String::from("diag1"), self.check_cut_state(&pos, diag1));
        res.insert(String::from("diag2"), self.check_cut_state(&pos, diag2));
        res 
    } 

    pub fn update_board(&mut self) {
        let mut mismatched = vec![];
        for (row_index, row) in self.board.iter().enumerate(){
            for (col_index, square) in row.iter().enumerate(){
                match square {
                    Some(s) => {
                        if s.get_cur_pos() != pieces::Pos(row_index.try_into().unwrap(), col_index.try_into().unwrap()) {
                            mismatched.push((row_index, col_index));
                        }
                    },
                    None => ()
                }
            }
        }
        for pos in mismatched{
            let (x,y) = pos;
            let p = self.board[x].remove(y);
            self.board[x].insert(y, None);
            self.put_piece(p.unwrap());
        }

        for x in 0..8 {
            for y in 0..8 {
                let state = self.get_state(pieces::Pos(x,y)).clone();
                let piece = &mut self.board[x as usize][y as usize];
                match piece {
                    Some(p) => {p.update_legal_moves(state);},
                    None => () 
                }
            }
        }
    }
}


