use moverox_traits::{MoveDatatype, MoveType};
use serde::{Deserialize, Serialize};

#[derive(MoveDatatype, Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[move_(nameless)]
pub struct Otw<T: MoveType>(bool, std::marker::PhantomData<T>);

fn main() {}
