use model::board::Board;

// declare the package model
mod model;
pub mod utils;

#[allow(dead_code)]
fn main() {
    println!("{}", Board::new_board());    
}
