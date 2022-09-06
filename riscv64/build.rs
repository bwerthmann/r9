// SPDX-License-Identifier: MPL-2.0
// SPDX-FileCopyrightText: 2022 The vanadinite developers
//
// This Source Code Form is subject to the terms of the Mozilla Public License,
// v. 2.0. If a copy of the MPL was not distributed with this file, You can
// obtain one at https://mozilla.org/MPL/2.0/.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "virt")]
    println!("cargo:rustc-link-arg=-Triscv64/lds/virt.ld");
    #[cfg(feature = "nezha")]
    println!("cargo:rustc-link-arg=-Triscv64/lds/nezha.ld");
    Ok(())
}
