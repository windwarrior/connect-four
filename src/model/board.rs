// Foreign imports
use std::fmt;
use std::num;
use std::result;

// Model imports
use model::marks::Mark;
use model::marks::Empty;

// Utils imports
use utils::transpose;

// Board size
pub static GRID_ROWS : uint = 6;
pub static GRID_COLUMNS : uint = 7;

#[deriving(Clone, Eq, PartialEq)]
pub struct Board {
    pub fields : Vec<Vec<Mark>>,
}

// Standard trait implementations
impl fmt::Show for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        let transposed = transpose(&self.fields);

        try!(write!(f, "======= Board =======\n"));

        for x in transposed.iter() {
            for y in x.iter() {
                try!(write!(f, "[{}]", y));
            }
            try!(write!(f, "\n"));
        }
        return Ok(());
    }
}

// Custom defined trait implementationts
// none at this moment

// Methods to the Board struct
impl Board {
    // Static members (alphabetical)
    pub fn new_board() -> Board {

        let mut fs = Vec::new();

        for _ in range(0,GRID_COLUMNS) {
            let mut row = Vec::new();
            for _ in range(0,GRID_ROWS) {
                row.push(Empty);
            }
            fs.push(row);
        }

        Board{fields: fs}
    }

    // Public members (alphabetical)

    /*
    * Drops a stone into a row, returns Err(&str) when an invalid move is tried
    */
    pub fn drop_mark(&mut self, column: uint, mark : Mark) -> result::Result<Option<Mark>, &'static str> {
        // First we need to decide whether this move can be done
        let loc = try!(self.get_free_slot(column));

        let result = self.move_result(column, mark);

        self.set(column, loc as uint, mark);

        Ok(result)
    }

    pub fn get_mark(&self, row : uint, col : uint) -> Result<Mark, &'static str> {
        if self.is_in_range(row, col) {
            Ok(self.fields[col][row])
        } else { 
            Err("Field outside of range")
        }
    }

    /*
    * Checks whether a move is valid
    */
    pub fn is_in_range(&self, row : uint, col : uint) -> bool {
        row < GRID_ROWS && col < GRID_COLUMNS
    }

    /*
    * Determines result of a move in a certain row
    * Does assume that the move has not yet been done
    */
    fn move_result(&self, column: uint, mark : Mark) -> Option<Mark> {

        let one = self.move_result_dir(column, mark, (1, 0));  // x axis
        let two = self.move_result_dir(column, mark, (0, 1));  // y axis
        let three = self.move_result_dir(column, mark, (1, 1));  // ascending diagonal
        let four = self.move_result_dir(column, mark, (1, -1)); // descending diagonal

        one.or(two.or(three.or(four)))
    }

    fn move_result_dir(&self, column: uint, mark: Mark, incs : (int, int)) -> Option<Mark> {
        let (mut lx, mut ly) = (column as int, self.get_free_slot(column).ok().unwrap() as int);
        let (mut ux, mut uy) = (lx, ly);

        let (xinc, yinc) = incs;

        let mut discovering = true;

        while discovering {
            discovering = false;

            let upper = self.get_mark((uy + yinc) as uint, (ux + xinc) as uint);

            if upper.is_ok() && upper.unwrap() == mark {
                ux = ux + xinc;
                uy = uy + yinc;
                discovering = true;
            }

            let lower = self.get_mark((ly - yinc) as uint, (lx - xinc) as uint);

            if lower.is_ok() && lower.unwrap() == mark {
                lx = lx - xinc;
                ly = ly - yinc;
                discovering = true;
            }
        }

        if num::abs(ux - lx) >= 3 || num::abs(uy - ly) >= 3 { Some(mark) } else { None }
    }

    fn get_free_slot(&self, column: uint) -> Result<uint, &'static str> {
        let mut loc = -1i;

        if column >= GRID_COLUMNS {
            return Err("Invalid column");
        }

        let ref vec_col = self.fields[column];
            

        for x in range(0, vec_col.len()) {
            if vec_col[x] == Empty {
                loc = x as int;
            }
        }

        if loc != -1 { Ok(loc as uint) } else { Err("No free slots") }
    }

    /*
    * Private function that just flips a field
    */
    fn set(&mut self, row : uint, col : uint, mark : Mark) {
        (*(*self.fields.get_mut(row)).get_mut(col))= mark;
    }
}
