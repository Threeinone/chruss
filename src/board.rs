pub struct Board([char; 64]);

impl std::ops::Index<usize> for Board
{
    type Output = char;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Board {
    pub fn new() -> Self {
        Board([
            'r','n','b','q','k','b','n','r',
            'p','p','p','p','p','p','p','p',
            '.','.','.','.','.','.','.','.',
            '.','.','.','.','.','.','.','.',
            '.','.','.','.','.','.','.','.',
            '.','.','.','.','.','.','.','.',
            'P','P','P','P','P','P','P','P',
            'R','N','B','Q','K','B','N','R',
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

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let board_printable = self.ranks().into_iter().enumerate();
        for (i, squares) in board_printable {
            let rank: i8 = (i as i8 - 8).abs();
            write!(f, "{rank}")?;
            squares
                .iter()
                .for_each(|sq| match write!(f, " {sq}") {
                    Err(_) => panic!("bad display somehow, idfk"),
                    _ => ()
                });
            write!(f, "\n")?;
        }
        write!(f, "  a b c d e f g h")
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
            ['r','n','b','q','k','b','n','r'],
            ['p','p','p','p','p','p','p','p'],
            ['.','.','.','.','.','.','.','.'],
            ['.','.','.','.','.','.','.','.'],
            ['.','.','.','.','.','.','.','.'],
            ['.','.','.','.','.','.','.','.'],
            ['P','P','P','P','P','P','P','P'],
            ['R','N','B','Q','K','B','N','R'],
        ]);
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
