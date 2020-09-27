// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::accessor::{Accessor, Bus, Device, Function, RegisterIndex};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub(crate) struct Registers {
    bus: Bus,
    device: Device,
}

impl Registers {
    pub(crate) fn fetch(bus: Bus, device: Device) -> Option<Self> {
        let registers = Self { bus, device };
        if !registers.exist() {
            None
        } else {
            Some(registers)
        }
    }

    pub(crate) fn get(&self, index: RegisterIndex) -> u32 {
        let accessor = Accessor::new(self.bus, self.device, Function::zero(), index);
        accessor.read()
    }

    fn exist(&self) -> bool {
        self.get(RegisterIndex::zero()) != !0
    }
}
