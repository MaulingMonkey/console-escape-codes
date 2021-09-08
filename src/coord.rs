use crate::*;
use std::fmt::{self, Debug, Display, Formatter};



/// 1-based row number
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)] pub struct RowNo(pub u32);

/// 0-based row index
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)] pub struct RowIdx(pub u32);

/// 1-based column number
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)] pub struct ColNo(pub u32);

/// 0-based column index
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)] pub struct ColIdx(pub u32);

/// 1-based row+column numbers
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)] pub struct RowColNo(pub u32, pub u32);

/// 0-based row+column indicies
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)] pub struct RowColIdx(pub u32, pub u32);



impl From<RowIdx    > for RowNo      { fn from(value: RowIdx      ) -> Self { Self(value.0 + u32::ONE) } }
impl From<ColIdx    > for ColNo      { fn from(value: ColIdx      ) -> Self { Self(value.0 + u32::ONE) } }
impl From<RowColIdx > for RowColNo   { fn from(value: RowColIdx   ) -> Self { Self(value.0 + u32::ONE, value.1 + u32::ONE) } }

impl Display for RowNo { fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { Display::fmt(&self.0, f) } }
impl Display for ColNo { fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { Display::fmt(&self.0, f) } }
