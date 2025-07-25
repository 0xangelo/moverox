use moverox_traits::MoveDatatype;
use serde::{Deserialize, Serialize};

#[derive(MoveDatatype, Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[move_(nameless)]
pub struct Otw(bool);

fn main() {}
