module enums::enums;

use std::string::String;

// Errors with:
// ```
// error[E02022]: invalid 'enum' declaration
//   ┌─ ./sources/main.move:3:1
//   │
// 3 │ public enum Empty {}
//   │ ^^^^^^^^^^^^^^^^^^^^ An 'enum' must define at least one variant
// ```
// public enum Empty {}

public enum Single {
    Only,
}

/// `Segment` enum definition.
/// Defines various string segments.
public enum Segment has copy, drop {
    /// Empty variant, no value.
    Empty,
    /// Variant with a value (positional style).
    String(String),
    /// Variant with named fields.
    Special {
        content: vector<u8>,
        encoding: u8, // Encoding tag.
    },
}

public enum Generic<phantom T> has copy, drop, store {
    Unit,
    Tuple(u64),
    Struct {
        value: u64,
    }
}
