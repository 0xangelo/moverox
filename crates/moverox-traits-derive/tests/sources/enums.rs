use moverox_traits::MoveDatatype;

#[derive(MoveDatatype)]
pub enum Single {
    Only,
}

/// `Segment` enum definition.
/// Defines various string segments.
#[derive(MoveDatatype)]
pub enum Segment {
    /// Empty variant, no value.
    Empty,
    /// Variant with a value (positional style).
    String(String),
    /// Variant with named fields.
    Special {
        content: Vec<u8>,
        encoding: u8, // Encoding tag.
    },
}

#[derive(MoveDatatype)]
pub enum Generic<T> {
    // Add a phantom data to the first variant to make the compiler happy
    Unit(std::marker::PhantomData<T>),
    Tuple(u64),
    Struct { value: u64 },
}

fn main() {}
