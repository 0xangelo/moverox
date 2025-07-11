use moverox_traits::MoveDatatype;

#[derive(MoveDatatype)]
#[move_(address = "0x2", module = balance)]
pub struct Balance<T> {
    value: u64,
    _otw: std::marker::PhantomData<T>,
}

fn main() {}
