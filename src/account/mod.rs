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

/* implement the cargo crate aleo-account v0.3.2 with #![no_std] without using standard libraries */
extern crate no_std_compat as std;
use std::prelude::v1::*;

// pub mod address;
// pub use address::*;

// pub mod private_key;
// pub use private_key::*;

pub mod signature;
pub use signature::*;

// pub mod view_key;
// pub use view_key::*;
