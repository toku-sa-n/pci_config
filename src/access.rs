// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub(crate) struct Access {
    bus: Bus,
    device: Device,
    function: Function,
    index: RegisterIndex,
}

impl Access {
    pub(crate) fn new(bus: Bus, device: Device, function: Function, index: RegisterIndex) -> Self {
        Self {
            bus,
            device,
            function,
            index,
        }
    }
}

#[derive(Copy, Clone)]
pub(crate) struct Bus(u8);
impl Bus {
    pub(crate) fn new(bus: u8) -> Self {
        Self(bus)
    }

    fn as_u8(self) -> u8 {
        self.0
    }
}

#[derive(Copy, Clone)]
pub(crate) struct Device(u8);
impl Device {
    pub(crate) fn new(device: u8) -> Self {
        assert!(device < 16);
        Self(device)
    }

    fn as_u8(self) -> u8 {
        self.0
    }
}

#[derive(Copy, Clone)]
pub(crate) struct Function(u8);
impl Function {
    pub(crate) fn new(function: u8) -> Self {
        assert!(function < 8);
        Self(function)
    }
}

#[derive(Copy, Clone)]
pub(crate) struct RegisterIndex(u8);
impl RegisterIndex {
    pub(crate) fn new(index: u8) -> Self {
        assert!(index < 64);
        Self(index)
    }
}
