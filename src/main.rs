use model::board::Board;

// declare the package model
mod model;
pub mod utils;

fn main() {
    println!("{}", Board::new_board());    
}
