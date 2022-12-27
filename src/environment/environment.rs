use no_std_compat::{fmt, hash};

// use crate::network::Network;
// use crate::types::field::Field;
// pub use once_cell::unsync::OnceCell;

pub trait Environment:
    'static + Copy + Clone + fmt::Debug + fmt::Display + Eq + PartialEq + hash::Hash
{
    // type Network: Network<
    //     Affine = Self::Affine,
    //     Field = Self::BaseField,
    //     Scalar = Self::ScalarField,
    // >;

    // type Affine: AffineCurve<
    //     BaseField = Self::BaseField,
    //     ScalarField = Self::ScalarField,
    //     Coordinates = (Self::BaseField, Self::BaseField),
    // >;
    // type BaseField: PrimeField + SquareRootField + Copy;
    // type ScalarField: PrimeField<BigInteger = <Self::BaseField as PrimeField>::BigInteger> + Copy;
}
