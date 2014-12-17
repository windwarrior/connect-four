use model::board::{Board, GRID_ROWS};
use model::marks::{Empty, Red, Yellow};

#[test]
fn test_get_mark_success() {
    let board = Board::new_board();

    assert_eq!(board.get_mark(0, 0), Ok(Empty));
}

#[test]
fn test_get_mark_fails() {
    let board = Board::new_board();

    assert!(board.get_mark(10, 0).is_err());
}

#[test]
fn test_get_mark_fails_negative() {
    let board = Board::new_board();

    assert!(board.get_mark(-10, 0).is_err());
}

#[test]
fn test_drop_mark_success() {
    let mut board = Board::new_board();

    assert!(board.drop_mark(0, Yellow).is_ok());
}

#[test]
fn test_drop_mark_fails() {
    let mut board = Board::new_board();

    assert!(board.drop_mark(10, Yellow).is_err());
}

#[test]
fn test_drop_mark_fails_negative() {
    let mut board = Board::new_board();

    assert!(board.drop_mark(-1, Yellow).is_err());
}

#[test]
fn test_drop_mark_full() {
    let mut board = Board::new_board();
    for i in range(0, GRID_ROWS) {
        board.drop_mark(0, Red);
    }

    assert!(board.drop_mark(0, Red).is_err());
}

#[test]
#[allow(unused_must_use)]
fn test_drop_mark_score_vertical_success(){
    let mut board = Board::new_board();

    board.drop_mark(0, Red);
    board.drop_mark(0, Red);
    board.drop_mark(0, Red);
    let res = board.drop_mark(0, Red);

    assert_eq!(res, Ok(Some(Red)));
}

#[test]
#[allow(unused_must_use)]
fn test_drop_mark_score_vertical_fail() {
    let mut board = Board::new_board();

    board.drop_mark(0, Red);
    board.drop_mark(0, Red);
    board.drop_mark(0, Yellow);
    let res = board.drop_mark(0, Red);

    assert_eq!(res, Ok(None));
}

#[test]
#[allow(unused_must_use)]
fn test_drop_mark_score_horizontal_success() {
    let mut board = Board::new_board();

    board.drop_mark(0, Red);
    board.drop_mark(1, Red);
    board.drop_mark(2, Red);
    let res = board.drop_mark(3, Red);

    assert_eq!(res, Ok(Some(Red)));
}

#[test]
#[allow(unused_must_use)]
fn test_drop_mark_score_horizontal_fail() {
    let mut board = Board::new_board();

    board.drop_mark(0, Red);
    board.drop_mark(1, Red);
    board.drop_mark(2, Yellow);
    let res = board.drop_mark(3, Red);

    assert_eq!(res, Ok(None));
}

#[test]
#[allow(unused_must_use)]
fn test_drop_mark_score_upper_diagonal_success() {
    let mut board = Board::new_board();

    board.drop_mark(0, Red);
    board.drop_mark(1, Yellow);
    board.drop_mark(1, Red);
    board.drop_mark(2, Yellow);
    board.drop_mark(2, Yellow);
    board.drop_mark(2, Red);
    board.drop_mark(3, Yellow);
    board.drop_mark(3, Yellow);
    board.drop_mark(3, Yellow);
    let res = board.drop_mark(3, Red);

    assert_eq!(res, Ok(Some(Red)));
}

#[test]
#[allow(unused_must_use)]
fn test_drop_mark_score_upper_diagonal_fail(){
    let mut board = Board::new_board();

    board.drop_mark(0, Red);
    board.drop_mark(1, Yellow);
    board.drop_mark(1, Red);
    board.drop_mark(2, Yellow);
    board.drop_mark(2, Yellow);
    board.drop_mark(2, Yellow);
    board.drop_mark(3, Yellow);
    board.drop_mark(3, Yellow);
    board.drop_mark(3, Yellow);
    let res = board.drop_mark(3, Red);

    assert_eq!(res, Ok(None));
}

#[test]
#[allow(unused_must_use)]
fn test_drop_mark_score_lower_diagonal_success() {
    let mut board = Board::new_board();

    board.drop_mark(0, Yellow);
    board.drop_mark(0, Yellow);
    board.drop_mark(0, Yellow);
    board.drop_mark(0, Red);
    board.drop_mark(1, Yellow);
    board.drop_mark(1, Yellow);
    board.drop_mark(1, Red);
    board.drop_mark(2, Yellow);
    board.drop_mark(2, Red);
    let res = board.drop_mark(3, Red);

    assert_eq!(res, Ok(Some(Red)));
}

#[test]
#[allow(unused_must_use)]
fn test_drop_mark_score_lower_diagonal_fail() {
    let mut board = Board::new_board();

    board.drop_mark(0, Yellow);
    board.drop_mark(0, Yellow);
    board.drop_mark(0, Yellow);
    board.drop_mark(0, Red);
    board.drop_mark(1, Yellow);
    board.drop_mark(1, Yellow);
    board.drop_mark(1, Yellow);
    board.drop_mark(2, Yellow);
    board.drop_mark(2, Red);
    let res = board.drop_mark(3, Red);

    assert_eq!(res, Ok(None));
}
