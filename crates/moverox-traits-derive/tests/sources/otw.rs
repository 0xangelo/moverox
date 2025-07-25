use moverox_traits::MoveDatatype;

#[derive(
    MoveDatatype, Clone, Debug, Default, serde::Deserialize, serde::Serialize, PartialEq, Eq, Hash,
)]
#[move_(nameless)]
pub struct Otw {
    dummy_field: bool,
}

fn main() {}
