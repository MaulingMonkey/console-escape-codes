use std::fmt::{self, Display, Formatter};



pub(crate) struct DisplayFn<F: Fn(&mut Formatter<'_>) -> fmt::Result>(pub F);

impl<F: Fn(&mut Formatter<'_>) -> fmt::Result> Display for DisplayFn<F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        (self.0)(f)
    }
}
