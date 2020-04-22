mod pieces;
mod board;
mod game;
use crate::pieces::Piece;

fn main() {
    let mut board: board::Board<pieces::Pawn>= board::Board::new();
    let mut white_pawns = vec![];
    for i in 0..8{
        let mut pawn = pieces::Pawn::new(false);
        pawn.init(i, board.get_state(pieces::Pos(6,i)));
        white_pawns.push(pawn);
    }

    for p in white_pawns.iter_mut(){
        board.put_piece(p);
    }

    let mut black_pawns = vec![];
    for i in 0..8{
        let mut pawn = pieces::Pawn::new(true);
        pawn.init(i, board.get_state(pieces::Pos(1,i)));
        black_pawns.push(pawn);
    }

    for p in black_pawns.iter_mut(){
        board.put_piece(p);
    }   
    
    //let state = board.get_state(pieces::Pos(3,3));
    //let mut bla = vec![];
    //println!("{:?}", state.get("diag1").unwrap());
    //for i in state.get("diag1").unwrap(){
    //    let mut p = pieces::Pawn::new(true);
    //    p.position = i.clone();
    //    bla.push(p);
    //}
    //for p in bla.iter_mut(){
    //    board.put_piece(p);
    //}
    //board.show();
    game::play(board);
}
