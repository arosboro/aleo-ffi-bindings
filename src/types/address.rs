use crate::environment::Environment;
use crate::types::group::Group;

pub struct Address<E: Environment> {
    /// The underlying address.
    address: Group<E>,
}
