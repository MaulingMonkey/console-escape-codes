use std::ops::*;



/// A placeholder for range expressions like <code>[Cursor](Cursor)[..](RangeFrom)</code>.  Often interchangeable with `()`.
pub struct Cursor;



/// <code>[Cursor](Cursor)[..](RangeFrom)</code> | <code>[..=](RangeToInclusive)[Cursor]</code> | <code>[..](RangeFull)</code>
pub trait BeforeAfterAllCursor { fn n012(&self) -> &'static str; }
impl BeforeAfterAllCursor for RangeFrom<Cursor>         { fn n012(&self) -> &'static str { "0" } }
impl BeforeAfterAllCursor for RangeToInclusive<Cursor>  { fn n012(&self) -> &'static str { "1" } }
impl BeforeAfterAllCursor for RangeFull                 { fn n012(&self) -> &'static str { "2" } }
impl BeforeAfterAllCursor for RangeFrom<()>             { fn n012(&self) -> &'static str { "0" } }
impl BeforeAfterAllCursor for RangeToInclusive<()>      { fn n012(&self) -> &'static str { "1" } }



/// <code>[Cursor]</code> | <code>[..](RangeFull)</code>
pub trait CursorOrAll           { fn is_all(&self) -> bool { false } }
impl CursorOrAll for ()         { fn is_all(&self) -> bool { false } }
impl CursorOrAll for Cursor     { fn is_all(&self) -> bool { false } }
impl CursorOrAll for RangeFull  { fn is_all(&self) -> bool { true  } }
