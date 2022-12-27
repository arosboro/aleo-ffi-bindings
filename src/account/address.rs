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
#![no_std]

use crate::account::signature_verify;
use crate::account::{PrivateKey, Signature, ViewKey};
use aleo_account::Address as AddressNative;

use core::{convert::TryFrom, fmt, ops::Deref, str::FromStr};

use ::safer_ffi::prelude::*;
use safer_ffi::std::string::ToString;
extern crate no_std_compat as std;
use std::prelude::v1::*;

#[derive_ReprC]
#[ReprC::opaque]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Address {
    native: AddressNative,
}

#[ffi_export]
pub(crate) fn address_from_private_key(
    private_key: &repr_c::Box<PrivateKey>,
) -> repr_c::Box<Address> {
    repr_c::Box::new(Address {
        native: AddressNative::try_from(**private_key.deref()).unwrap(),
    })
}

#[ffi_export]
pub(crate) fn address_from_view_key(view_key: &repr_c::Box<ViewKey>) -> repr_c::Box<Address> {
    repr_c::Box::new(Address {
        native: AddressNative::try_from(**view_key.deref()).unwrap(),
    })
}

#[ffi_export]
pub fn address_from_string(address: char_p::Ref<'_>) -> repr_c::Box<Address> {
    repr_c::Box::new(Address::from_str(address.to_str()).unwrap())
}

#[ffi_export]
pub(crate) fn address_to_string(address: &Address) -> char_p::Box {
    address.native.to_string().try_into().unwrap()
}

#[ffi_export]
pub fn address_verify(
    address: &repr_c::Box<Address>,
    message: c_slice::Ref<u8>,
    signature: &repr_c::Box<Signature>,
) -> repr_c::Box<bool> {
    signature_verify(&signature, &address, message)
}

impl FromStr for Address {
    type Err = anyhow::Error;

    fn from_str(address: &str) -> Result<Self, Self::Err> {
        Ok(Address {
            native: AddressNative::from_str(address)?,
        })
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.native)
    }
}

impl Deref for Address {
    type Target = AddressNative;

    fn deref(&self) -> &Self::Target {
        &self.native
    }
}

// fn destroy(address: repr_c::Box<Address>) {
//     drop(address)
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::account::private_key_new;
    use crate::account::private_key_to_view_key;

    const ITERATIONS: u64 = 1_000;

    #[test]
    pub fn test_from_private_key() {
        for _ in 0..ITERATIONS {
            // Sample a new private key.
            let private_key = private_key_new();
            let expected = address_from_private_key(&private_key);

            // Check the address derived from the view key.
            let view_key = private_key_to_view_key(&private_key);
            assert_eq!(expected.native, address_from_view_key(&view_key).native);
        }
    }
}
