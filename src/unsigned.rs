use std::fmt::Display;
use std::ops::*;



/// [`u8`] | [`u16`] | [`u32`] | [`u64`] | [`u128`] | [`std::num::NonZeroU*`](https://doc.rust-lang.org/std/num/struct.NonZeroI8.html?search=std%3A%3Anum%3A%3ANonZeroU)
pub trait Unsigned : Display + Clone + Copy + Add<Output = Self> + 'static { const ONE : Self; }
impl Unsigned for u8            { const ONE : Self = 1; }
impl Unsigned for u16           { const ONE : Self = 1; }
impl Unsigned for u32           { const ONE : Self = 1; }
impl Unsigned for u64           { const ONE : Self = 1; }
impl Unsigned for u128          { const ONE : Self = 1; }
