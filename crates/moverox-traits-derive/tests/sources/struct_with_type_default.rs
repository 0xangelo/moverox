use moverox_traits::MoveDatatype;

#[derive(MoveDatatype)]
#[move_(address = "0x2", module = balance)]
pub struct Balance<T = Otw> {
    value: u64,
    _otw: std::marker::PhantomData<T>,
}

#[derive(
    MoveDatatype, Clone, Debug, Default, serde::Deserialize, serde::Serialize, PartialEq, Eq, Hash,
)]
#[move_(nameless)]
pub struct Otw {
    dummy_field: bool,
}

fn main() {}
