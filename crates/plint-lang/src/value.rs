use std::range::Range;

pub enum Value {
    Match(Range<usize>),
    Number(f64),
    String(String),
}
