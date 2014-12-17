use std::fmt;
use std::clone;


pub fn transpose<T : fmt::Show + clone::Clone>(vec : &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let x = vec.len(); // The amount of columns
    let y = vec[0].len();

    let mut res : Vec<Vec<T>> = Vec::new();

    for i in range(0, y) {
        let mut row = Vec::new();

        for j in range(0, x) {
            let a = vec[j][i].clone();

            row.push(a);
        }

        res.push(row);
    }

    return res;
}
