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

use crate::account::address_from_view_key;
use crate::account::{Address, PrivateKey};
use aleo_account::{Record, ViewKey as ViewKeyNative};

use safer_ffi::Tuple2;

use core::{convert::TryFrom, fmt, ops::Deref, str::FromStr};

use ::safer_ffi::prelude::*;
use safer_ffi::std::string::ToString;
extern crate no_std_compat as std;
use std::prelude::v1::*;

#[derive_ReprC]
#[ReprC::opaque]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ViewKey {
    native: ViewKeyNative,
}

#[ffi_export]
pub(crate) fn view_key_from_private_key(
    private_key: &repr_c::Box<PrivateKey>,
) -> repr_c::Box<ViewKey> {
    repr_c::Box::new(ViewKey {
        native: ViewKeyNative::try_from(**private_key.deref()).unwrap(),
    })
}

#[ffi_export]
pub fn view_key_from_string(view_key: char_p::Ref<'_>) -> repr_c::Box<ViewKey> {
    repr_c::Box::new(ViewKey::from_str(view_key.to_str()).unwrap())
}

#[ffi_export]
pub(crate) fn view_key_to_string(view_key: &repr_c::Box<ViewKey>) -> char_p::Box {
    view_key.native.to_string().try_into().unwrap()
}

#[ffi_export]
pub fn view_key_to_address(view_key: &repr_c::Box<ViewKey>) -> repr_c::Box<Address> {
    address_from_view_key(view_key)
}

#[ffi_export]
pub fn view_key_decrypt(
    view_key: &repr_c::Box<ViewKey>,
    ciphertext: char_p::Ref<'_>,
) -> Tuple2<repr_c::Box<bool>, char_p::Box> {
    let ciphertext = Record::from_str(ciphertext.to_str())
        .map_err(|error| error.to_string())
        .unwrap();
    match ciphertext.decrypt(&view_key.native) {
        Ok(plaintext) => Tuple2 {
            _0: repr_c::Box::new(true),
            _1: plaintext.to_string().try_into().unwrap(),
        },
        Err(error) => Tuple2 {
            _0: repr_c::Box::new(false),
            _1: error.to_string().try_into().unwrap(),
        },
    }
}

impl FromStr for ViewKey {
    type Err = anyhow::Error;

    fn from_str(view_key: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            native: ViewKeyNative::from_str(view_key)?,
        })
    }
}

impl fmt::Display for ViewKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.native)
    }
}

impl Deref for ViewKey {
    type Target = ViewKeyNative;

    fn deref(&self) -> &Self::Target {
        &self.native
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::account::private_key_from_string;

    const OWNER_PLAINTEXT: &str = r"{
  owner: aleo1y50whk20gjtltkte2qcqz9dd6uaet8thhlj3t8utewp0j3hhmg8qae7s5a.public,
  gates: 1159017656332810u64.public,
  a: 6875465154712544327395236939215127424077297244802949502285127742492653680374field.private,
  b: 603076889203566020456049671526074557206943911693533670547825725507132399266scalar.private,
  _nonce: 1635890755607797813652478911794003479783620859881520791852904112255813473142group.public
}";
    const OWNER_CIPHERTEXT: &str = "record1qqj3a67efazf0awe09grqqg44htnh9vaw7l729vl309c972x7ldquqq2k2cax8s7qsqqyqtpgvqqyqsq4seyrzvfa98fkggzccqr68af8e9m0q8rzeqh8a8aqql3a854v58sgrygdv4jn9s8ckwfd48vujrmv0rtfasqh8ygn88ch34ftck8szspvfpsqqszqzvxx9t8s9g66teeepgxmvnw5ymgapcwt2lpy9d5eus580k08wpq544jcl437wjv206u5pxst6few9ll4yhufwldgpx80rlwq8nhssqywmfsd85skg564vqhm3gxsp8q6r30udmqxrxmxx2v8xycdg8pn5ps3dhfvv";
    const OWNER_VIEW_KEY: &str = "AViewKey1ghtvuJQQzQ31xSiVh6X1PK8biEVhQBygRGV4KdYmq4JT";
    const NON_OWNER_VIEW_KEY: &str = "AViewKey1e2WyreaH5H4RBcioLL2GnxvHk5Ud46EtwycnhTdXLmXp";

    #[test]
    pub fn test_from_private_key() {
        let given_private_key = "APrivateKey1zkp4RyQ8Utj7aRcJgPQGEok8RMzWwUZzBhhgX6rhmBT8dcP";
        let given_view_key = "AViewKey1i3fn5SECcVBtQMCVtTPSvdApoMYmg3ToJfNDfgHJAuoD";
        let private_key = private_key_from_string(char_p::new(given_private_key).as_ref());
        let view_key = view_key_from_private_key(&private_key);
        assert_eq!(given_view_key, view_key.to_string());
    }

    #[test]
    pub fn test_decrypt_success() {
        let view_key = view_key_from_string(char_p::new(OWNER_VIEW_KEY).as_ref());
        let plaintext = view_key_decrypt(&view_key, char_p::new(OWNER_CIPHERTEXT).as_ref());
        assert!(plaintext._0.deref());
        assert_eq!(OWNER_PLAINTEXT, plaintext._1.to_str())
    }

    #[test]
    pub fn test_decrypt_fails() {
        let ciphertext = Record::from_str(OWNER_CIPHERTEXT)
            .map_err(|error| error.to_string())
            .unwrap();
        let incorrect_view_key = view_key_from_string(char_p::new(NON_OWNER_VIEW_KEY).as_ref());
        let plaintext = ciphertext.decrypt(&incorrect_view_key.deref());
        assert!(plaintext.is_err());
    }
}
