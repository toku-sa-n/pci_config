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

    fn address(&self) -> u32 {
        let bus = self.bus.as_u8() as u32;
        let device = self.device.as_u8() as u32;
        let function = self.device.as_u8() as u32;
        let index = self.index.as_u8() as u32;

        const VALID: u32 = 0x8000_0000;

        VALID | bus << 16 | device << 11 | function << 8 | index << 2
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

    fn as_u8(self) -> u8 {
        self.0
    }
}

#[derive(Copy, Clone)]
pub(crate) struct RegisterIndex(u8);
impl RegisterIndex {
    pub(crate) fn new(index: u8) -> Self {
        assert!(index < 64);
        Self(index)
    }

    fn as_u8(self) -> u8 {
        self.0
    }
}
