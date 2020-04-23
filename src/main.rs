mod pieces;
mod board;
mod game;
use crate::pieces::Piece;

fn main() {
    let mut board: board::Board<pieces::Rook>= board::Board::new();
    let mut white_pieces = vec![];
    //for i in 0..8{
    //    let mut pawn = pieces::Pawn::new(false);
    //    pawn.init(i, board.get_state(pieces::Pos(6,i)));
    //    white_pieces.push(Box::new(pawn));
    //}

    for i in 0..2{
        let mut rook = pieces::Rook::new(false);
        let y = if i == 0 {0} else {7};
        rook.init(i, board.get_state(pieces::Pos(7, y)));
        white_pieces.push(Box::new(rook));
    }

    for p in white_pieces.iter_mut(){
        board.put_piece(p);
    }

    let mut black_pieces = vec![];
    //for i in 0..8{
    //    let mut pawn = pieces::Pawn::new(true);
    //    pawn.init(i, board.get_state(pieces::Pos(1,i)));
    //    black_pieces.push(Box::new(pawn));
    //}

    for i in 0..2{
        let mut rook = pieces::Rook::new(true);
        let y = if i == 0 {0} else {7};
        rook.init(i, board.get_state(pieces::Pos(0, y)));
        black_pieces.push(Box::new(rook));
    }

    for p in black_pieces.iter_mut(){
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
