use std::fmt::{Display, Formatter};
use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct InvalidInput;

impl Display for InvalidInput {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "An answer couldn't be found from the input, or the input was malformed.")
    }
}
impl Error for InvalidInput {}
