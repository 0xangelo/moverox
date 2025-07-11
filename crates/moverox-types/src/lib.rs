#![cfg_attr(all(doc, not(doctest)), feature(doc_auto_cfg))]

//! Building blocks for oxidized Move types.

mod address;
mod ident_str;
mod type_tag;
pub mod u256;

pub use self::address::{Address, AddressParseError};
pub use self::ident_str::{IdentStr, InvalidIdentifierError};
pub use self::type_tag::{Identifier, StructTag, TypeParseError, TypeTag};
#[doc(inline)]
pub use self::u256::U256;

/// Contruct an [`Address`] from hex bytes at compile time.
///
/// # Example
/// ```
/// use moverox_types::{Address, const_address};
///
/// const FRAMEWORK: Address = const_address(b"0x2");
/// ```
pub const fn const_address(bytes: &[u8]) -> Address {
    Address::new(self::comp_time::hex_array(bytes))
}

mod comp_time {
    pub(super) const fn hex_array<const T: usize>(bytes: &[u8]) -> [u8; T] {
        use const_hex::{FromHexError as E, const_decode_to_array};
        let padded = strip_and_pad::<T>(bytes);
        match const_decode_to_array(padded.as_flattened()) {
            Ok(arr) => arr,
            Err(E::OddLength) => panic!("Odd length"),
            Err(E::InvalidStringLength) => panic!("Invalid string length"),
            Err(E::InvalidHexCharacter { .. }) => panic!("Invalid hex character"),
        }
    }

    const fn strip_and_pad<const T: usize>(bytes: &[u8]) -> [[u8; T]; 2] {
        let stripped = match bytes {
            [b'0', b'x', rest @ ..] => rest,
            _ => bytes,
        };

        let mut matrix = [[b'0'; T], [b'0'; T]];
        let padded = matrix.as_flattened_mut();
        if padded.len() < stripped.len() {
            panic!("String too long");
        }
        let tail_start = padded.len() - stripped.len();
        padded.split_at_mut(tail_start).1.copy_from_slice(stripped);
        matrix
    }

    #[test]
    fn preprocessing() {
        let bytes = b"0x2";
        let processed = strip_and_pad::<32>(bytes);
        assert_eq!(
            processed.as_flattened(),
            b"0000000000000000000000000000000000000000000000000000000000000002"
        )
    }
}
