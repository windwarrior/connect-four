use std::fmt;

#[deriving(Clone, PartialEq, Eq)]
pub enum Mark {
    Red,
    Yellow,
    Empty
}

impl fmt::Show for Mark {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Red => write!(f, "r"),
            Yellow => write!(f, "y"),
            Empty => write!(f, "-")
        }
    }
}
