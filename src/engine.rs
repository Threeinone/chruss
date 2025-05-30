use std::slice::SliceIndex;
use std::ops::Index;

pub struct Board([char; 64]);

impl<I> Index<I> for Board
where I: SliceIndex<[char], Output = char>,
{
    type Output = char;

    fn index(&self, index: I) -> &Self::Output {
        &self.0[index]
    }
}

impl Board {
    pub fn new() -> Self {
        Board([
            'R','N','B','Q','K','B','N','R',
            'P','P','P','P','P','P','P','P',
            '.','.','.','.','.','.','.','.',
            '.','.','.','.','.','.','.','.',
            '.','.','.','.','.','.','.','.',
            '.','.','.','.','.','.','.','.',
            'p','p','p','p','p','p','p','p',
            'r','n','b','q','k','b','n','r',
        ])
    }
    pub fn files(&self) -> Vec<Vec<char>> {
        let mut ret = Vec::new();
        for file_offset in 0..8 {
            let current_file = (0..8).map(|sq_offset| self[file_offset + (sq_offset * 8)]);
            ret.push(current_file.collect());
        }
        ret
    }
    pub fn ranks(&self) -> Vec<Vec<char>> {
        let mut ret = Vec::new();
        for rank_offset in 0..8 {
            let current_rank = (0..8).map(|sq_offset| self[(rank_offset * 8) + sq_offset]);
            ret.push(current_rank.collect());
        }
        ret
    }
}

/*
pub fn print_board(board: Board, show_coords: bool) {
    let board_printable = board.ranks().into_iter().enumerate();
    for (i, squares) in board_printable {
        if show_coords {
            let rank: i8 = (i as i8 - 8).abs();
            print!("{rank}")
        }
        squares
            .iter()
            .for_each(|sq| print!(" {sq}") );
        print!("\n");
    }
    if show_coords {
        println!("  a b c d e f g h");
    }
}
*/

// FIXME: I just reversed the board's ordering, so this will be wrong
impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let board_printable = self.ranks().into_iter().enumerate();
        for (i, squares) in board_printable {
            let rank: i8 = (i as i8 - 8).abs();
            write!(f, "{rank}")?;
            /*
            squares
                .iter()
                .for_each(|sq| match write!(f, " {sq}") {
                    Err(_) => panic!("bad display somehow, idfk"),
                    _ => ()
                });
            */
            for sq in squares {
                write!(f, " {sq}")?;
            }
            write!(f, "\n")?;
        }
        write!(f, "  a b c d e f g h\n")
    }
}

// check what is on a certain square of the board using algebraic notation
// example: index_board("e1") -> 'K'
//fn index_board(board: Board, square: &str) -> char

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ranks_and_files() {
        let board = Board::new();
        assert_eq!(board.ranks(), [
            ['R','N','B','Q','K','B','N','R'],
            ['P','P','P','P','P','P','P','P'],
            ['.','.','.','.','.','.','.','.'],
            ['.','.','.','.','.','.','.','.'],
            ['.','.','.','.','.','.','.','.'],
            ['.','.','.','.','.','.','.','.'],
            ['p','p','p','p','p','p','p','p'],
            ['r','n','b','q','k','b','n','r'],
        ]);
        // FIXME: the pieces are backwards now
        assert_eq!(board.files(), [
            ['r','p','.','.','.','.','P','R'],
            ['n','p','.','.','.','.','P','N'],
            ['b','p','.','.','.','.','P','B'],
            ['q','p','.','.','.','.','P','Q'],
            ['k','p','.','.','.','.','P','K'],
            ['b','p','.','.','.','.','P','B'],
            ['n','p','.','.','.','.','P','N'],
            ['r','p','.','.','.','.','P','R'],
        ]);
    }
}

pub struct GameTurn( pub String, pub Option<String> );
pub struct GameHistory(Vec<GameTurn>);

impl GameTurn {

    pub fn from(white_move: String, black_move: Option<String>) -> Self {
        GameTurn(white_move, black_move)
    }
}

impl GameHistory {

    pub fn new() -> Self {
        GameHistory(Vec::new())
    }

    pub fn get_turn<I>(&self, turn: usize) -> Option<&GameTurn> {
        if turn > self.0.len() {
            return None;
        }

        Some(&self.0[turn-1])
    }
    pub fn push_turn(&mut self, new_turn: GameTurn) {
        self.0.push(new_turn);
    }
}

#[derive(PartialEq)]
pub enum ToMove {
    White,
    Black
}

