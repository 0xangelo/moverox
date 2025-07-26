module enums::other;

use enums::enums::Single;

public enum Wrapper<T> {
    Single(Single),
    OtherPositional(T),
    OtherNamed {
        inner: T,
    }
}
