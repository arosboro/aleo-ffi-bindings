use crate::environment::{Environment, LinearCombination};
// use crate::types::boolean::Boolean;
// use once_cell::sync::OnceCell;

pub struct Field<E: Environment> {
    linear_combination: LinearCombination<E::BaseField>,
    // bits_le: OnceCell<Vec<Boolean>>,
}
