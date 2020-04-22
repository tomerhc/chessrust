use std::collections::HashMap;

pub trait Piece {
    fn new(color: bool) -> Self;
    fn init(&mut self, num:u8, board_state: HashMap<String, Vec<Pos>>);
    fn move_piece(&mut self, board_state: HashMap<String, Vec<Pos>>, pos: Pos) -> bool; 
    fn calc_legal_moves(&self, board_state: HashMap<String, Vec<Pos>>) -> Option<Vec<Pos>>;
    fn show(&self) -> String;
    fn set_init_pos(&self, num: u8) -> Pos;
    fn get_cur_pos(&self) -> Pos;
    fn get_color(&self) -> bool;
    fn get_piece_type(&self) -> String;
    fn update_legal_moves(&mut self, board_state: HashMap<String, Vec<Pos>>);
    fn is_my_group<T: Piece>(&self, other: &T) -> bool {
        self.get_color() == other.get_color()
    }
    fn is_on_board(pos: &Pos) -> bool {
        let Pos(x,y) = pos;
        0 < *x && *x < 8 && 0 < *y && *y<8
    }
    

}

#[derive(Debug, Clone, PartialEq)]
pub struct Pos(pub u8,pub u8);

pub struct Pawn {
    pub color: bool,
    pub position: Pos,
    pub legal_moves: Option<Vec<Pos>>,
    pub piece_type: String,
}

impl Piece for Pawn {
    fn new(color:bool) -> Pawn {
        Pawn {
            color: color,
            position: Pos(0,0),
            legal_moves: None,
            piece_type: String::from("pawn")
        }
    }

    fn init(&mut self, num: u8, board_state: HashMap<String, Vec<Pos>>){
        self.position = self.set_init_pos(num);
        self.legal_moves = self.calc_legal_moves(board_state);
    }

    fn set_init_pos(&self, num: u8) -> Pos {
        match self.color{
            true => Pos(6, num), //that means white
            false => Pos(1, num)
        }
    }

    fn calc_legal_moves(&self, mut board_state: HashMap<String, Vec<Pos>>) -> Option<Vec<Pos>> {
        let mut possible = vec![];
        let Pos(my_x, my_y) = self.position;
        if self.color {
            //for white: 
            if &board_state.get("diag1").unwrap().len() > &0{
                let top_right = &board_state.get("diag1").unwrap()[0];
                let Pos(x,_) = *top_right;
                if x > my_x && my_y < 7 {
                    possible.push(Pos(my_x-1, my_y+1));
                }
            }

            if &board_state.get("diag2").unwrap().len() > &0 {
                let top_left = &board_state.get("diag2").unwrap()[0];
                let Pos(x,_) = *top_left;
                if x > my_x && my_y > 0{
                    possible.push(Pos(my_x-1, my_y-1));
                }
            }

            for i in board_state.get("col").unwrap().into_iter(){
                let Pos(x,y) = *i;
                if x == my_x-1 && y == my_y {
                    possible.push(Pos(x,y));
                    break;
                }
            }

        }else{

            //for black:
            let diag1_rev = board_state.get_mut("diag1").unwrap();
            if diag1_rev.len() > 0{
                diag1_rev.reverse();

                let bottom_left = &diag1_rev[0];
                let Pos(x,_) =* bottom_left;
                if x < my_x && my_y > 0 {
                    possible.push(Pos(my_x+1, my_y-1))
                }
            }

            let diag2_rev = board_state.get_mut("diag2").unwrap();
            if diag2_rev.len() > 0 {
                diag2_rev.reverse();

                let bottom_right = &diag2_rev[0];
                let Pos(x,_) = *bottom_right;
                if x < my_x && my_y < 7{
                    possible.push(Pos(my_x+1, my_y+1))
                }
            }

            for i in board_state.get("col").unwrap().into_iter(){
                let Pos(x,y) = *i;
                if x == my_x+1 && y == my_y {                    
                    possible.push(Pos(x,y));
                    break;
                }
            }
        }
       Some(possible) 
    }

    fn update_legal_moves(&mut self, board_state: HashMap<String, Vec<Pos>>) {
        self.legal_moves = self.calc_legal_moves(board_state);
    }


    fn move_piece(&mut self, board_state: HashMap<String, Vec<Pos>>, pos: Pos) -> bool{
        let Pos(x_new, y_new) = pos;
        match &self.legal_moves {
            Some(v) => { 
                if v.iter().any(|&Pos(x,y)| x==x_new && y==y_new){
                    self.position = pos;
                    self.legal_moves = self.calc_legal_moves(board_state);
                    return true;
                } else {
                    println!("illigal move");
                    return false;
                }
            },
            None => {
                println!("illigal move");
                return false;
            }
        }
    }

    fn show(&self) -> String{
        if self.color{
            String::from("x")
        }else{
            String::from("X")
        }
    }

    fn get_cur_pos(&self) -> Pos {
        self.position.clone()
    }

    fn get_color(&self) -> bool {
        self.color
    }

    fn get_piece_type(&self) -> String {
        self.piece_type.clone()
    }
}
