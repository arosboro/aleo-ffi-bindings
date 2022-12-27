use crate::environment::{Environment, LinearCombination};

pub struct Boolean<E: Environment>(LinearCombination<E::BaseField>);
