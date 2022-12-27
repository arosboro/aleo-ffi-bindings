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
#![no_std]
#![deny(unsafe_code)]

// pub mod environment;

// pub mod network;

// pub mod types;

// pub mod utilities;

pub mod account;

// The following test function is necessary for the header generation.
// #[::safer_ffi::cfg_headers]
// #[test]
// fn generate_headers() -> ::std::io::Result<()> {
//     ::safer_ffi::headers::builder()
//         .to_file("libaleo.h")?
//         .generate()
// }
