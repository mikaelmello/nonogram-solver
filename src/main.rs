use board::Board;
use eyre::Result;
use strategy::{Strategy, UnambiguousConstraints};

mod board;
mod strategy;

fn main() -> Result<()> {
    let sz: usize = inquire::CustomType::new("Board length:").prompt()?;

    let mut board = Board::new(sz, vec![vec![1]; sz], vec![vec![sz]; sz]);

    println!("{:?}", board);

    solve(&mut board);

    println!("{:?}", board);

    Ok(())
}

fn solve(board: &mut Board) {
    UnambiguousConstraints::apply(board);
}
