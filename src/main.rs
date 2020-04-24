mod pieces;
mod board;
mod game;
use crate::pieces::Piece;

fn main() {
    let mut board = board::Board::new();
    let mut white_pawns= vec![];
    for i in 0..8{
        let mut pawn = pieces::Pawn::new(false);
        pawn.init(i, board.get_state(pieces::Pos(6,i)));
        white_pawns.push(pawn);
    }

    let mut white_rooks= vec![];
    for i in 0..2{
        let mut rook = pieces::Rook::new(false);
        let y = if i == 0 {0} else {7};
        rook.init(i, board.get_state(pieces::Pos(7, y)));
        white_rooks.push(rook);
    }

    let mut white_bisops= vec![];
    for i in 0..2{
        let mut bisop = pieces::Bisop::new(false);
        let y = if i == 0 {0} else {7};
        bisop.init(i, board.get_state(pieces::Pos(7, y)));
        white_bisops.push(bisop);
    }

    let mut white_knights= vec![];
    for i in 0..2{
        let mut knight = pieces::Knight::new(false);
        let y = if i == 0 {0} else {7};
        knight.init(i, board.get_state(pieces::Pos(7, y)));
        white_knights.push(knight);
    }

    let length  = white_pawns.len();
    for _ in 0..length {
        board.put_piece(Box::new(white_pawns.pop().unwrap()));
    }
    
    let length  = white_rooks.len();
    for _ in 0..length {
        board.put_piece(Box::new(white_rooks.pop().unwrap()));
    }

    let length  = white_bisops.len();
    for _ in 0..length {
        board.put_piece(Box::new(white_bisops.pop().unwrap()));
    }

    let mut white_queen = pieces::Queen::new(false);
    white_queen.init(0, board.get_state(pieces::Pos(0,5)));
    board.put_piece(Box::new(white_queen));
    let mut white_king = pieces::King::new(false);
    white_king.init(0, board.get_state(pieces::Pos(0,5)));
    board.put_piece(Box::new(white_king));
    
    let length  = white_knights.len();
    for _ in 0..length {
        board.put_piece(Box::new(white_knights.pop().unwrap()));
    }

    let mut black_pawns = vec![];
    for i in 0..8{
        let mut pawn = pieces::Pawn::new(true);
        pawn.init(i, board.get_state(pieces::Pos(1,i)));
        black_pawns.push(pawn);
    }

    let mut black_rooks = vec![];
    for i in 0..2{
        let mut rook = pieces::Rook::new(true);
        let y = if i == 0 {0} else {7};
        rook.init(i, board.get_state(pieces::Pos(0, y)));
        black_rooks.push(rook);
    }
    
    let mut black_bisops = vec![];
    for i in 0..2{
        let mut bisop = pieces::Bisop::new(true);
        let y = if i == 0 {0} else {7};
        bisop.init(i, board.get_state(pieces::Pos(0, y)));
        black_bisops.push(bisop);
    }

    let mut black_knights = vec![];
    for i in 0..2{
        let mut knight = pieces::Knight::new(true);
        let y = if i == 0 {0} else {7};
        knight.init(i, board.get_state(pieces::Pos(0, y)));
        black_knights.push(knight);
    }


    let mut black_queen = pieces::Queen::new(true);
    black_queen.init(0, board.get_state(pieces::Pos(7,5)));
    board.put_piece(Box::new(black_queen));
    let mut black_king = pieces::King::new(true);
    black_king.init(0, board.get_state(pieces::Pos(7,5)));
    board.put_piece(Box::new(black_king));
    
    let length  = black_pawns.len();
    for _ in 0..length {
        board.put_piece(Box::new(black_pawns.pop().unwrap()));
    }

    let length  = black_rooks.len();
    for _ in 0..length {
        board.put_piece(Box::new(black_rooks.pop().unwrap()));
    }
    
    let length  = black_bisops.len();
    for _ in 0..length {
        board.put_piece(Box::new(black_bisops.pop().unwrap()));
    }

    let length  = black_knights.len();
    for _ in 0..length {
        board.put_piece(Box::new(black_knights.pop().unwrap()));
    }
    //let state = board.get_state(pieces::Pos(3,3));
    //let mut bla = vec![];
    //println!("{:?}", state.get("diag1").unwrap());
    //for i in state.get("diag1").unwrap(){
    //    let mut p = pieces::Pawn::new(true);
    //    p.position = i.clone();
    //    bla.push(p);
    //}
    //for p in bla.iter(){
    //    board.put_piece(p);
    //}
    //board.show();
    game::play(board);
}
