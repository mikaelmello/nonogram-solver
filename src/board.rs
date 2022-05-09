use std::{collections::HashSet, fmt::Debug};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Nil,
    NonFilled,
    Filled,
}

pub struct Board {
    len: usize,
    row_constraints: Vec<Vec<usize>>,
    col_constraints: Vec<Vec<usize>>,
    cells: Vec<Vec<Cell>>,
    solved_rows: HashSet<usize>,
    solved_cols: HashSet<usize>,
}

impl Board {
    pub fn new(
        len: usize,
        row_constraints: Vec<Vec<usize>>,
        col_constraints: Vec<Vec<usize>>,
    ) -> Board {
        assert_eq!(
            len,
            row_constraints.len(),
            "Wrong number of row constraints"
        );
        assert_eq!(
            len,
            col_constraints.len(),
            "Wrong number of col constraints"
        );

        let validate_constraints = |kind: &str, idx: usize, c: &[usize]| {
            assert!(!c.is_empty(), "{} {} must have constraints", kind, idx);

            let sum: usize = c.iter().sum();
            let total = sum + c.len() - 1;

            assert!(
                total <= len,
                "Total sum of checked boxes + separators must be less than length in {} {}",
                kind,
                idx
            );
        };

        row_constraints
            .iter()
            .enumerate()
            .for_each(|(idx, c)| validate_constraints("row", idx, c));

        col_constraints
            .iter()
            .enumerate()
            .for_each(|(idx, c)| validate_constraints("col", idx, c));

        Board {
            len,
            row_constraints,
            col_constraints,
            cells: vec![vec![Cell::Nil; len]; len],
            solved_cols: HashSet::new(),
            solved_rows: HashSet::new(),
        }
    }

    pub fn get_len(&self) -> usize {
        self.len
    }

    pub fn get_cell(&self, i: usize, j: usize) -> Cell {
        assert!(i < self.len);
        assert!(j < self.len);

        self.cells[i][j]
    }

    pub fn get_row_constraints(&self, i: usize) -> &[usize] {
        assert!(i < self.len);

        &self.row_constraints[i]
    }

    pub fn get_col_constraints(&self, j: usize) -> &[usize] {
        assert!(j < self.len);

        &self.col_constraints[j]
    }

    pub fn set_cell(&mut self, i: usize, j: usize, value: Cell) {
        assert!(i < self.len);
        assert!(j < self.len);
        assert_ne!(value, Cell::Nil);
        assert_eq!(self.cells[i][j], Cell::Nil);

        self.cells[i][j] = value;
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = (self.len + 1) * 2 + 1;
        writeln!(f, "{:=<1$}", "", width)?;

        for r in 0..self.len {
            write!(f, "| ")?;

            for c in 0..self.len {
                let ch = match self.cells[r][c] {
                    Cell::Filled => '■',
                    Cell::NonFilled => 'x',
                    Cell::Nil => '_',
                };
                write!(f, "{} ", ch)?;
            }

            writeln!(f, "|")?;
        }

        writeln!(f, "{:=<1$}", "", width)?;
        Ok(())
    }
}
