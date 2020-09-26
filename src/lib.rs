// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![no_std]
#![deny(clippy::all)]

mod space;

use {
    core::iter::Iterator,
    space::{
        accessor::{Bus, Device},
        registers::Registers,
        Space,
    },
};

struct IterDevices {
    bus: u8,
    device: u8,
}

impl IterDevices {
    fn new(bus: u8, device: u8) -> Self {
        Self { bus, device }
    }
}

impl Iterator for IterDevices {
    type Item = Space;

    fn next(&mut self) -> Option<Self::Item> {
        for bus in self.bus as usize..Bus::MAX {
            for device in self.device..Device::MAX {
                if let Some(registers) = Registers::fetch(Bus::new(bus as u8), Device::new(device))
                {
                    self.bus = bus as u8;
                    self.device = device as u8 + 1;

                    return Some(Space::parse_registers(&registers));
                }
            }
        }

        None
    }
}
