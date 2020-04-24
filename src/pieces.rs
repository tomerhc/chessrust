use std::collections::HashMap;

pub trait Piece {
//    fn new(color: bool) -> Self;
    fn init(&mut self, num:u8, board_state: HashMap<String, Vec<Pos>>);
    fn move_piece(&mut self, board_state: HashMap<String, Vec<Pos>>, pos: Pos) -> bool; 
    fn calc_legal_moves(&self, board_state: HashMap<String, Vec<Pos>>) -> Option<Vec<Pos>>;
    fn show(&self) -> String;
    fn set_init_pos(&self, num: u8) -> Pos;
    fn get_cur_pos(&self) -> Pos;
    fn get_color(&self) -> bool;
    fn get_piece_type(&self) -> String;
    fn update_legal_moves(&mut self, board_state: HashMap<String, Vec<Pos>>);
//    fn is_my_group<T: Piece>(&self, other: &T) -> bool {
//        self.get_color() == other.get_color()
//    }
//    fn is_on_board(pos: &Pos) -> bool {
//        let Pos(x,y) = pos;
//        0 < *x && *x < 8 && 0 < *y && *y<8
//    }
//    

}

#[derive(Debug, Clone, PartialEq)]
pub struct Pos(pub u8,pub u8);

pub struct Pawn {
    pub color: bool,
    pub position: Pos,
    pub legal_moves: Option<Vec<Pos>>,
    pub piece_type: String,
}

impl Pawn {
    pub fn new(color:bool) -> Pawn {
        Pawn {
            color: color,
            position: Pos(0,0),
            legal_moves: None,
            piece_type: String::from("pawn")
        }
    }

}

impl Piece for Pawn {

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
            }else if my_y < 7 {
                possible.push(Pos(my_x-1, my_y+1));
            }

            if &board_state.get("diag2").unwrap().len() > &0 {
                let top_left = &board_state.get("diag2").unwrap()[0];
                let Pos(x,_) = *top_left;
                if x > my_x && my_y > 0{
                    possible.push(Pos(my_x-1, my_y-1));
                }
            }else if my_y > 0{
                possible.push(Pos(my_x-1, my_y-1));
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
            }else if my_y > 0 {
                possible.push(Pos(my_x+1, my_y-1))
            }

            let diag2_rev = board_state.get_mut("diag2").unwrap();
            if diag2_rev.len() > 0 {
                diag2_rev.reverse();

                let bottom_right = &diag2_rev[0];
                let Pos(x,_) = *bottom_right;
                if x < my_x && my_y < 7{
                    possible.push(Pos(my_x+1, my_y+1))
                }
            }else if my_y < 7{
                possible.push(Pos(my_x+1, my_y+1))
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



pub struct Rook {
    pub color: bool,
    pub position: Pos,
    pub legal_moves: Option<Vec<Pos>>,
    pub piece_type: String,
}

impl Rook {
    pub fn new(color:bool) -> Rook {
        Rook {
            color: color,
            position: Pos(0,0),
            legal_moves: None,
            piece_type: String::from("rook")
        }
    }
}


impl Piece for Rook {

    fn init(&mut self, num: u8, board_state: HashMap<String, Vec<Pos>>){
        self.position = self.set_init_pos(num);
        self.legal_moves = self.calc_legal_moves(board_state);
    }

    fn set_init_pos(&self, num: u8) -> Pos {
        let y = if num == 0 {0} else {7};
        match self.color{
            true => Pos(7, y as u8),
            false => Pos(0, y as u8)
        }
    }

    fn calc_legal_moves(&self, board_state: HashMap<String, Vec<Pos>>) -> Option<Vec<Pos>> {
        let mut possible = vec![];
        let Pos(my_x, my_y) = self.position;
        let col_len = board_state.get("col").unwrap().len();
        let row_len = board_state.get("row").unwrap().len();

        if col_len == 0 {
            if my_x > 0 {
                possible.push(Pos(my_x-1, my_y));
            }
            if my_x < 7 {
                possible.push(Pos(my_x+1, my_y));
            }
        }

        if row_len == 0 {
            if my_y > 0 {
                possible.push(Pos(my_x, my_y-1));
            }
            if my_y < 7 {
                possible.push(Pos(my_x, my_y+1));
            }
        }


        for (index, i) in board_state.get("col").unwrap().into_iter().enumerate(){
            let Pos(x,y) = *i;
            if index == 0 && x > 0 {
                possible.push(Pos(x-1,y));
            }
            if index == col_len-1 && x < 7 {
                possible.push(Pos(x+1,y));
            }
            
            possible.push(Pos(x,y));
        }
        
        for (index, i) in board_state.get("row").unwrap().into_iter().enumerate(){
            let Pos(x,y) = *i;
            if index == 0 && y > 0 {
                possible.push(Pos(x,y-1));
            }
            if index == row_len-1 && y < 7 {
                possible.push(Pos(x,y+1));
            }
            
            possible.push(Pos(x,y));
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
            String::from("r")
        }else{
            String::from("R")
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

pub struct Bisop {
    pub color: bool,
    pub position: Pos,
    pub legal_moves: Option<Vec<Pos>>,
    pub piece_type: String,
}

impl Bisop {
    pub fn new(color:bool) -> Bisop {
        Bisop {
            color: color,
            position: Pos(0,0),
            legal_moves: None,
            piece_type: String::from("bisop")
        }
    }
}


impl Piece for Bisop {

    fn init(&mut self, num: u8, board_state: HashMap<String, Vec<Pos>>){
        self.position = self.set_init_pos(num);
        self.legal_moves = self.calc_legal_moves(board_state);
    }

    fn set_init_pos(&self, num: u8) -> Pos {
        let y = if num == 0 {2} else {5};
        match self.color{
            true => Pos(7, y as u8),
            false => Pos(0, y as u8)
        }
    }

    fn calc_legal_moves(&self, board_state: HashMap<String, Vec<Pos>>) -> Option<Vec<Pos>> {
        let mut possible = vec![];
        let Pos(my_x, my_y) = self.position;
        let diag1_len = board_state.get("diag1").unwrap().len();
        let diag2_len = board_state.get("diag2").unwrap().len();

        if diag1_len == 0 {
            if my_x > 0 && my_y < 7{
                possible.push(Pos(my_x-1, my_y+1));
            }
            if my_x < 7 && my_y > 0{
                possible.push(Pos(my_x+1, my_y-1));
            }
        }

        if diag2_len == 0 {
            if my_y > 0 && my_x > 0{
                possible.push(Pos(my_x-1, my_y-1));
            }
            if my_y < 7 && my_x < 7{
                possible.push(Pos(my_x+1, my_y+1));
            }
        }

        for (index, i) in board_state.get("diag1").unwrap().into_iter().enumerate(){
            let Pos(x,y) = *i;
            if index == 0 && x > 0 && y < 7 {
                possible.push(Pos(x-1,y+1));
            }
            if index == diag1_len-1 && x < 7 && y > 7 {
                possible.push(Pos(x+1,y-1));
            }
            
            possible.push(Pos(x,y));
        }
        
        for (index, i) in board_state.get("diag2").unwrap().into_iter().enumerate(){
            let Pos(x,y) = *i;
            if index == 0 && y > 0 && x > 0{
                possible.push(Pos(x-1, y-1));
            }
            if index == diag2_len-1 && y < 7 && x < 7{
                possible.push(Pos(x+1, y+1));
            }
            
            possible.push(Pos(x,y));
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
            String::from("b")
        }else{
            String::from("B")
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

pub struct Knight {
    pub color: bool,
    pub position: Pos,
    pub legal_moves: Option<Vec<Pos>>,
    pub piece_type: String,
}

impl Knight {
    pub fn new(color:bool) -> Knight {
        Knight {
            color: color,
            position: Pos(0,0),
            legal_moves: None,
            piece_type: String::from("knight")
        }
    }
}


impl Piece for Knight {

    fn init(&mut self, num: u8, board_state: HashMap<String, Vec<Pos>>){
        self.position = self.set_init_pos(num);
        self.legal_moves = self.calc_legal_moves(board_state);
    }

    fn set_init_pos(&self, num: u8) -> Pos {
        let y = if num == 0 {1} else {6};
        match self.color{
            true => Pos(7, y as u8),
            false => Pos(0, y as u8)
        }
    }

    fn calc_legal_moves(&self, board_state: HashMap<String, Vec<Pos>>) -> Option<Vec<Pos>> {
        let mut possible = vec![];
        let Pos(x, y) = self.position;
        let my_x  = x as i8;
        let my_y  = y as i8;

        possible.push((my_x -2, my_y+1));
        possible.push((my_x -2, my_y-1));
        possible.push((my_x +2, my_y+1));
        possible.push((my_x +2, my_y-1));
        possible.push((my_x +1, my_y-2));
        possible.push((my_x +1, my_y+2));
        possible.push((my_x -1, my_y-2));
        possible.push((my_x -1, my_y+2));
        
        let final_pos: Vec<Pos> = possible.into_iter().filter(|(x,y)| *x > 0 && *x < 8 && *y > 0 && *y < 8).map(|(x,y)| Pos(x as u8, y as u8)).collect();
        Some(final_pos) 
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
            String::from("n")
        }else{
            String::from("N")
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

pub struct Queen {
    pub color: bool,
    pub position: Pos,
    pub legal_moves: Option<Vec<Pos>>,
    pub piece_type: String,
}

impl Queen {
    pub fn new(color:bool) -> Queen {
        Queen {
            color: color,
            position: Pos(0,0),
            legal_moves: None,
            piece_type: String::from("queen")
        }
    }
}


impl Piece for Queen {

    fn init(&mut self, num: u8, board_state: HashMap<String, Vec<Pos>>){
        self.position = self.set_init_pos(num);
        self.legal_moves = self.calc_legal_moves(board_state);
    }

    fn set_init_pos(&self, num: u8) -> Pos {
        match self.color{
            true => Pos(7, 3),
            false => Pos(0, 3)
        }
    }

    fn calc_legal_moves(&self, board_state: HashMap<String, Vec<Pos>>) -> Option<Vec<Pos>> {
        let mut possible = vec![];
        let Pos(my_x, my_y) = self.position;
        let col_len = board_state.get("col").unwrap().len();
        let row_len = board_state.get("row").unwrap().len();

        if col_len == 0 {
            if my_x > 0 {
                possible.push(Pos(my_x-1, my_y));
            }
            if my_x < 7 {
                possible.push(Pos(my_x+1, my_y));
            }
        }

        if row_len == 0 {
            if my_y > 0 {
                possible.push(Pos(my_x, my_y-1));
            }
            if my_y < 7 {
                possible.push(Pos(my_x, my_y+1));
            }
        }


        for (index, i) in board_state.get("col").unwrap().into_iter().enumerate(){
            let Pos(x,y) = *i;
            if index == 0 && x > 0 {
                possible.push(Pos(x-1,y));
            }
            if index == col_len-1 && x < 7 {
                possible.push(Pos(x+1,y));
            }
            
            possible.push(Pos(x,y));
        }
        
        for (index, i) in board_state.get("row").unwrap().into_iter().enumerate(){
            let Pos(x,y) = *i;
            if index == 0 && y > 0 {
                possible.push(Pos(x,y-1));
            }
            if index == row_len-1 && y < 7 {
                possible.push(Pos(x,y+1));
            }
            
            possible.push(Pos(x,y));
        }


        let diag1_len = board_state.get("diag1").unwrap().len();
        let diag2_len = board_state.get("diag2").unwrap().len();

        if diag1_len == 0 {
            if my_x > 0 && my_y < 7{
                possible.push(Pos(my_x-1, my_y+1));
            }
            if my_x < 7 && my_y > 0{
                possible.push(Pos(my_x+1, my_y-1));
            }
        }

        if diag2_len == 0 {
            if my_y > 0 && my_x > 0{
                possible.push(Pos(my_x-1, my_y-1));
            }
            if my_y < 7 && my_x < 7{
                possible.push(Pos(my_x+1, my_y+1));
            }
        }

        for (index, i) in board_state.get("diag1").unwrap().into_iter().enumerate(){
            let Pos(x,y) = *i;
            if index == 0 && x > 0 && y < 7 {
                possible.push(Pos(x-1,y+1));
            }
            if index == diag1_len-1 && x < 7 && y > 7 {
                possible.push(Pos(x+1,y-1));
            }
            
            possible.push(Pos(x,y));
        }
        
        for (index, i) in board_state.get("diag2").unwrap().into_iter().enumerate(){
            let Pos(x,y) = *i;
            if index == 0 && y > 0 && x > 0{
                possible.push(Pos(x-1, y-1));
            }
            if index == diag2_len-1 && y < 7 && x < 7{
                possible.push(Pos(x+1, y+1));
            }
            
            possible.push(Pos(x,y));
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
            String::from("q")
        }else{
            String::from("Q")
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

pub struct King {
    pub color: bool,
    pub position: Pos,
    pub legal_moves: Option<Vec<Pos>>,
    pub piece_type: String,
}

impl King {
    pub fn new(color:bool) -> King {
        King {
            color: color,
            position: Pos(0,0),
            legal_moves: None,
            piece_type: String::from("king")
        }
    }
}


impl Piece for King {

    fn init(&mut self, num: u8, board_state: HashMap<String, Vec<Pos>>){
        self.position = self.set_init_pos(num);
        self.legal_moves = self.calc_legal_moves(board_state);
    }

    fn set_init_pos(&self, num: u8) -> Pos {
        match self.color{
            true => Pos(7,4),
            false => Pos(0, 4)
        }
    }   

    fn calc_legal_moves(&self, board_state: HashMap<String, Vec<Pos>>) -> Option<Vec<Pos>> {
        let mut possible = vec![];
        let Pos(x, y) = self.position;
        let my_x  = x as i8;
        let my_y  = y as i8;

        possible.push((my_x-1, my_y));
        possible.push((my_x+1, my_y));
        possible.push((my_x, my_y-1));
        possible.push((my_x, my_y+1));
        possible.push((my_x+1, my_y+1));
        possible.push((my_x+1, my_y-1));
        possible.push((my_x-1, my_y+1));
        possible.push((my_x-1, my_y-1));
        
        let final_pos: Vec<Pos> = possible.into_iter().filter(|(x,y)| *x > 0 && *x < 8 && *y > 0 && *y < 8).map(|(x,y)| Pos(x as u8, y as u8)).collect();
        Some(final_pos) 
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
            String::from("k")
        }else{
            String::from("K")
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

