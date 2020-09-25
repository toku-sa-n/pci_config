// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![no_std]
#![deny(clippy::all)]

struct Bus(u8);
impl Bus {
    fn new(bus: u8) -> Self {
        Self(bus)
    }
}

struct Device(u8);
impl Device {
    fn new(device: u8) -> Self {
        assert!(device < 16);
        Self(device)
    }
}

struct Function(u8);
impl Function {
    fn new(function: u8) -> Self {
        assert!(function < 8);
        Self(function)
    }
}
