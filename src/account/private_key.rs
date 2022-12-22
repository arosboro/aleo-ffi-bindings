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

use crate::account::address_from_private_key;
use crate::account::signature_sign;
use crate::account::view_key_from_private_key;
use crate::account::{Address, Signature, ViewKey};
use aleo_account::{
    CurrentNetwork, Environment, FromBytes, PrimeField, PrivateKey as PrivateKeyNative, ToBytes,
};

use core::{convert::TryInto, fmt, ops::Deref, str::FromStr};
use rand::{rngs::StdRng, SeedableRng};

use ::safer_ffi::prelude::*;
use safer_ffi::std::string::ToString;

#[derive_ReprC]
#[ReprC::opaque]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrivateKey {
    native: PrivateKeyNative,
}

#[ffi_export]
pub(crate) fn private_key_new() -> repr_c::Box<PrivateKey> {
    console_error_panic_hook::set_once();
    repr_c::Box::new(PrivateKey {
        native: PrivateKeyNative::new(&mut StdRng::from_entropy()).unwrap(),
    })
}

#[ffi_export]
pub fn private_key_from_seed_unchecked(seed: c_slice::Ref<u8>) -> repr_c::Box<PrivateKey> {
    console_error_panic_hook::set_once();
    // Cast into a fixed-size byte array. Note: This is a **hard** requirement for security.
    let seed: [u8; 32] = seed.to_vec().try_into().unwrap();
    // Recover the field element deterministically.
    let field = <CurrentNetwork as Environment>::Field::from_bytes_le_mod_order(&seed);
    // Cast and recover the private key from the seed.
    repr_c::Box::new(PrivateKey {
        native: PrivateKeyNative::try_from(
            FromBytes::read_le(&*field.to_bytes_le().unwrap()).unwrap(),
        )
        .unwrap(),
    })
}

#[ffi_export]
pub(crate) fn private_key_from_string(private_key: char_p::Ref<'_>) -> repr_c::Box<PrivateKey> {
    repr_c::Box::new(PrivateKey::from_str(private_key.to_str()).unwrap())
}

#[ffi_export]
pub(crate) fn private_key_to_string(private_key: &repr_c::Box<PrivateKey>) -> char_p::Box {
    private_key.to_string().try_into().unwrap()
}

#[ffi_export]
pub fn private_key_to_view_key(private_key: &repr_c::Box<PrivateKey>) -> repr_c::Box<ViewKey> {
    view_key_from_private_key(private_key)
}

#[ffi_export]
pub(crate) fn private_key_to_address(
    private_key: &repr_c::Box<PrivateKey>,
) -> repr_c::Box<Address> {
    address_from_private_key(private_key)
}

#[ffi_export]
pub fn private_key_sign(
    private_key: &repr_c::Box<PrivateKey>,
    message: c_slice::Ref<u8>,
) -> repr_c::Box<Signature> {
    signature_sign(private_key, message)
}

impl FromStr for PrivateKey {
    type Err = anyhow::Error;

    fn from_str(private_key: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            native: PrivateKeyNative::from_str(private_key)?,
        })
    }
}

impl fmt::Display for PrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.native)
    }
}

impl Deref for PrivateKey {
    type Target = PrivateKeyNative;

    fn deref(&self) -> &Self::Target {
        &self.native
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use rand::Rng;

    use crate::account::address_from_view_key;
    use crate::account::address_to_string;
    use crate::account::signature_verify;
    use crate::account::view_key_to_string;

    const ITERATIONS: u64 = 1_000;

    const ALEO_PRIVATE_KEY: &str = "APrivateKey1zkp3dQx4WASWYQVWKkq14v3RoQDfY2kbLssUj7iifi1VUQ6";
    const ALEO_VIEW_KEY: &str = "AViewKey1cxguxtKkjYnT9XDza9yTvVMxt6Ckb1Pv4ck1hppMzmCB";
    const ALEO_ADDRESS: &str = "aleo184vuwr5u7u0ha5f5k44067dd2uaqewxx6pe5ltha5pv99wvhfqxqv339h4";

    #[test]
    pub fn test_sanity_check() {
        let private_key = private_key_from_string(char_p::new(ALEO_PRIVATE_KEY).as_ref());

        // println!(
        //     "{} == {}",
        //     ALEO_PRIVATE_KEY,
        //     private_key_to_string(&private_key).to_str()
        // );
        assert_eq!(
            ALEO_PRIVATE_KEY,
            private_key_to_string(&private_key).to_str()
        );

        // println!(
        //     "{} == {}",
        //     ALEO_VIEW_KEY,
        //     view_key_to_string(&private_key_to_view_key(&private_key)).to_str()
        // );
        assert_eq!(
            ALEO_VIEW_KEY,
            view_key_to_string(&private_key_to_view_key(&private_key)).to_str()
        );

        // println!(
        //     "{} == {}",
        //     ALEO_ADDRESS,
        //     address_to_string(&private_key_to_address(&private_key)).to_str()
        // );
        assert_eq!(
            ALEO_ADDRESS,
            address_to_string(&private_key_to_address(&private_key)).to_str()
        );
    }

    #[test]
    pub fn test_new() {
        for _ in 0..ITERATIONS {
            // Generate a new private_key.
            let expected = private_key_new();

            // Check the private_key derived from string.
            assert_eq!(
                expected.native,
                private_key_from_string(private_key_to_string(&expected).as_ref()).native
            );
        }
    }

    #[test]
    pub fn test_from_seed_unchecked() {
        for _ in 0..ITERATIONS {
            // Sample a random seed.
            let seed: [u8; 32] = StdRng::from_entropy().gen();

            // Ensure the private key is deterministically recoverable.
            let expected = private_key_from_seed_unchecked(c_slice::Ref::from(seed.as_slice()));
            assert_eq!(
                expected.native,
                private_key_from_seed_unchecked(c_slice::Ref::from(seed.as_slice())).native
            );
        }
    }

    #[test]
    pub fn test_to_address() {
        for _ in 0..ITERATIONS {
            // Sample a new private key.
            let private_key = private_key_new();
            let expected = private_key_to_address(&private_key);

            // Check the private_key derived from the view key.
            let view_key = private_key_to_view_key(&private_key);
            assert_eq!(expected.deref(), address_from_view_key(&view_key).deref());
        }
    }

    #[test]
    pub fn test_signature() {
        for _ in 0..ITERATIONS {
            // Sample a new private key and message.
            let private_key = private_key_new();
            let message: [u8; 32] = StdRng::from_entropy().gen();

            // Sign the message.
            let signature = private_key_sign(&private_key, c_slice::Ref::from(message.as_slice()));
            // Check the signature is valid.
            assert!(signature_verify(
                &signature,
                &private_key_to_address(&private_key),
                c_slice::Ref::from(message.as_slice())
            )
            .deref());
            // Check the signature is valid (natively).
            assert!(signature
                .deref()
                .verify_bytes(&private_key_to_address(&private_key).deref(), &message));
        }
    }
}
