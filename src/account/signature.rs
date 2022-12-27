// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the Aleo library.

// The Aleo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Aleo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Aleo library. If not, see <https://www.gnu.org/licenses/>.

#![deny(unsafe_code)]

// use crate::account::{Address, PrivateKey};
use snarkvm_wasm::network::Testnet3;
use snarkvm_wasm::Signature as AleoSignature;
// pub use snarkvm_wasm::{network::Environment, FromBytes, PrimeField, ToBytes};
pub type CurrentNetwork = Testnet3;
pub type SignatureNative = AleoSignature<CurrentNetwork>;

// use core::{fmt, ops::Deref, str::FromStr};
// use rand::{rngs::StdRng, SeedableRng};

// use ::safer_ffi::prelude::*;
// use safer_ffi::std::string::ToString;
extern crate no_std_compat as std;
use std::prelude::v1::*;

// #[derive_ReprC]
// #[ReprC::opaque]
pub struct Signature {
    native: SignatureNative,
}

// #[ffi_export]
// pub(crate) fn signature_sign(
//     private_key: &repr_c::Box<PrivateKey>,
//     message: c_slice::Ref<u8>,
// ) -> repr_c::Box<Signature> {
//     repr_c::Box::new(Signature {
//         native: SignatureNative::sign_bytes(
//             private_key,
//             message.deref(),
//             &mut StdRng::from_entropy(),
//         )
//         .unwrap(),
//     })
// }

// #[ffi_export]
// pub(crate) fn signature_verify(
//     signature: &repr_c::Box<Signature>,
//     address: &repr_c::Box<Address>,
//     message: c_slice::Ref<u8>,
// ) -> repr_c::Box<bool> {
//     repr_c::Box::new(
//         signature
//             .native
//             .verify_bytes(&address.deref(), message.deref()),
//     )
// }

// // #[ffi_export]
// pub fn signature_from_string(signature: char_p::Ref<'_>) -> repr_c::Box<Signature> {
//     repr_c::Box::new(Signature::from_str(signature.to_str()).unwrap())
// }

// // #[ffi_export]
// pub fn signature_to_string(signature: &repr_c::Box<Signature>) -> char_p::Box {
//     signature.native.to_string().try_into().unwrap()
// }

// impl FromStr for Signature {
//     type Err = anyhow::Error;

//     fn from_str(signature: &str) -> Result<Self, Self::Err> {
//         Ok(Self {
//             native: SignatureNative::from_str(signature)?,
//         })
//     }
// }

// impl fmt::Display for Signature {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.native)
//     }
// }

// impl Deref for Signature {
//     type Target = SignatureNative;

//     fn deref(&self) -> &Self::Target {
//         &self.native
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::account::private_key_new;
//     use crate::account::private_key_to_address;

//     use rand::{rngs::StdRng, Rng, SeedableRng};

//     const ITERATIONS: u64 = 1_000;

//     #[test]
//     pub fn test_sign_and_verify() {
//         for _ in 0..ITERATIONS {
//             // Sample a new private key and message.
//             let private_key = private_key_new();
//             let message: [u8; 32] = StdRng::from_entropy().gen();

//             // Sign the message.
//             let signature = signature_sign(&private_key, c_slice::Ref::from(message.as_slice()));
//             // Check the signature is valid.
//             assert!(signature_verify(
//                 &signature,
//                 &private_key_to_address(&private_key),
//                 c_slice::Ref::from(message.as_slice())
//             )
//             .deref());

//             // Sample a different message.
//             let bad_message: [u8; 32] = StdRng::from_entropy().gen();
//             // Check the signature is invalid.
//             assert!(!signature_verify(
//                 &signature,
//                 &private_key_to_address(&private_key),
//                 c_slice::Ref::from(bad_message.as_slice())
//             )
//             .deref());
//         }
//     }
// }
