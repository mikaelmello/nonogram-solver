use crate::board::{Board, Cell};

pub trait Strategy {
    fn apply(board: &mut Board);
}

pub struct UnambiguousConstraints;

impl Strategy for UnambiguousConstraints {
    fn apply(board: &mut Board) {
        let len = board.get_len();

        for r in 0..len {
            let constraints = board.get_row_constraints(r).to_vec();

            let sum: usize = constraints.iter().sum();
            let total = sum + constraints.len() - 1;

            if total != len {
                continue;
            }

            let mut idx = 0;
            for c in constraints {
                for _ in 0..c {
                    match board.get_cell(r, idx) {
                        Cell::Nil => board.set_cell(r, idx, Cell::Filled),
                        Cell::NonFilled => panic!("Had to fill but it is already blocked!"),
                        Cell::Filled => {}
                    }
                    idx += 1;
                }

                if idx == len {
                    break;
                }

                match board.get_cell(r, idx) {
                    Cell::Nil => board.set_cell(r, idx, Cell::Filled),
                    Cell::Filled => panic!("Had to fill but it is already blocked!"),
                    Cell::NonFilled => {}
                }

                idx += 1;
            }
        }

        for col in 0..len {
            let constraints = board.get_col_constraints(col).to_vec();

            let sum: usize = constraints.iter().sum();
            let total = sum + constraints.len() - 1;

            if total != len {
                continue;
            }

            let mut idx = 0;
            for c in constraints {
                for _ in 0..c {
                    match board.get_cell(idx, col) {
                        Cell::Nil => board.set_cell(idx, col, Cell::Filled),
                        Cell::NonFilled => panic!("Had to fill but it is already blocked!"),
                        Cell::Filled => {}
                    }
                    idx += 1;
                }

                if idx == len {
                    break;
                }

                match board.get_cell(idx, col) {
                    Cell::Nil => board.set_cell(idx, col, Cell::Filled),
                    Cell::NonFilled => {}
                    Cell::Filled => panic!("Had to block but it is already filled!"),
                }
                idx += 1;
            }
        }
    }
}
