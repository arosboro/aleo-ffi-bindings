use super::field::Field;
use crate::environment::Environment;

pub struct Group<E: Environment> {
    x: Field<E>,
    y: Field<E>,
}
