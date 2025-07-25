use moverox_traits::{MoveDatatype, MoveType};

#[derive(MoveDatatype)]
#[move_(address = "0x2", module = balance)]
pub struct Balance<T: MoveType> {
    value: u64,
    _otw: std::marker::PhantomData<T>,
}

fn main() {}
