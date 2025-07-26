module enums::other;

use enums::enums::Single;

public enum Wrapper<T> {
    Single(Single),
    OtherPositional(T),
    OtherNamed {
        inner: T,
    }
}

/// Type parameter `Single` should shadow the imported type with the same name.
public enum Shadowed<Single> {
    Only(Single)
}
