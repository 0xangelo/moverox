// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/// Corresponds to the `address` type in Move.
///
/// It is a 32-byte pseudonymous identifier used to uniquely identify an account and asset-ownership
/// on the blockchain.
///
/// Often, human-readable addresses are encoded in hexadecimal with a `0x` prefix. For example, this
/// is a valid address: `0x02a212de6a9dfa3a69e22387acfbafbb1a9e591bd9d636e7895dcfc8de05f331`.
///
/// ```
/// use moverox_types::Address;
///
/// let hex = "0x02a212de6a9dfa3a69e22387acfbafbb1a9e591bd9d636e7895dcfc8de05f331";
/// let address = Address::from_hex(hex).unwrap();
/// println!("Address: {}", address);
/// assert_eq!(hex, address.to_string());
/// ```
///
/// # BCS
///
/// An `Address`'s BCS serialized form is defined by the following:
///
/// ```text
/// address = 32OCTET
/// ```
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "proptest", derive(test_strategy::Arbitrary))]
pub struct Address(
    #[cfg_attr(
        feature = "serde",
        serde(with = "::serde_with::As::<::serde_with::IfIsHumanReadable<ReadableAddress>>")
    )]
    [u8; Self::LENGTH],
);

impl Address {
    pub const LENGTH: usize = 32;
    pub const ZERO: Self = Self([0u8; Self::LENGTH]);
    pub const TWO: Self = Self::from_u8(2);
    pub const THREE: Self = Self::from_u8(3);

    pub const fn new(bytes: [u8; Self::LENGTH]) -> Self {
        Self(bytes)
    }

    const fn from_u8(byte: u8) -> Self {
        let mut address = Self::ZERO;
        address.0[31] = byte;
        address
    }

    /// Return the underlying byte array of a Address.
    pub const fn into_inner(self) -> [u8; Self::LENGTH] {
        self.0
    }

    pub const fn inner(&self) -> &[u8; Self::LENGTH] {
        &self.0
    }

    pub const fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, AddressParseError> {
        let hex = hex.as_ref();

        if !hex.starts_with(b"0x") {
            return Err(AddressParseError);
        }

        let hex = &hex[2..];

        // If the string is too short we'll need to pad with 0's
        if hex.len() < Self::LENGTH * 2 {
            let mut buf = [b'0'; Self::LENGTH * 2];
            let pad_length = (Self::LENGTH * 2) - hex.len();

            buf[pad_length..].copy_from_slice(hex);

            <[u8; Self::LENGTH] as const_hex::FromHex>::from_hex(buf)
        } else {
            <[u8; Self::LENGTH] as const_hex::FromHex>::from_hex(hex)
        }
        .map(Self)
        //TODO fix error to contain hex parse error
        .map_err(|_| AddressParseError)
    }

    pub fn as_hex(&self) -> String {
        self.to_string()
    }

    pub fn from_bytes<T: AsRef<[u8]>>(bytes: T) -> Result<Self, AddressParseError> {
        <[u8; Self::LENGTH]>::try_from(bytes.as_ref())
            .map_err(|_| AddressParseError)
            .map(Self)
    }
}

impl std::str::FromStr for Address {
    type Err = AddressParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_hex(s)
    }
}

impl AsRef<[u8]> for Address {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8; 32]> for Address {
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

impl From<Address> for [u8; 32] {
    fn from(address: Address) -> Self {
        address.into_inner()
    }
}

impl From<[u8; 32]> for Address {
    fn from(address: [u8; 32]) -> Self {
        Self::new(address)
    }
}

impl From<Address> for Vec<u8> {
    fn from(value: Address) -> Self {
        value.0.to_vec()
    }
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x")?;
        for byte in &self.0 {
            write!(f, "{byte:02x}")?;
        }

        Ok(())
    }
}

impl std::fmt::Debug for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Address")
            .field(&format_args!("\"{self}\""))
            .finish()
    }
}

#[cfg(feature = "serde")]
struct ReadableAddress;

#[cfg(feature = "serde")]
impl serde_with::SerializeAs<[u8; Address::LENGTH]> for ReadableAddress {
    fn serialize_as<S>(source: &[u8; Address::LENGTH], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let address = Address::new(*source);
        serde_with::DisplayFromStr::serialize_as(&address, serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde_with::DeserializeAs<'de, [u8; Address::LENGTH]> for ReadableAddress {
    fn deserialize_as<D>(deserializer: D) -> Result<[u8; Address::LENGTH], D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let address: Address = serde_with::DisplayFromStr::deserialize_as(deserializer)?;
        Ok(address.into_inner())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AddressParseError;

impl std::fmt::Display for AddressParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Unable to parse Address (must be hex string of length {})",
            Address::LENGTH
        )
    }
}

impl std::error::Error for AddressParseError {}

#[cfg(test)]
mod test {
    use test_strategy::proptest;

    use super::*;

    #[test]
    fn hex_parsing() {
        let actual = Address::from_hex("0x2").unwrap();
        let expected = "0x0000000000000000000000000000000000000000000000000000000000000002";

        assert_eq!(actual.to_string(), expected);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn formats() {
        let actual = Address::from_hex("0x2").unwrap();

        println!("{}", serde_json::to_string(&actual).unwrap());
        println!("{:?}", bcs::to_bytes(&actual).unwrap());
        let a: Address = serde_json::from_str("\"0x2\"").unwrap();
        println!("{a}");
    }

    #[proptest]
    fn roundtrip_display_fromstr(address: Address) {
        let s = address.to_string();
        let a = s.parse::<Address>().unwrap();
        assert_eq!(address, a);
    }
}
