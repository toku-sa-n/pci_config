// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use {
    super::accessor::{Accessor, Bus, Device, Function, RegisterIndex},
    core::ops::Index,
};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub(crate) struct Registers([u32; RegisterIndex::MAX]);
impl Registers {
    pub(crate) fn fetch(bus: Bus, device: Device) -> Option<Self> {
        if !Self::exist(bus, device) {
            None
        } else {
            let mut registers = [0u32; RegisterIndex::MAX];

            for (i, register) in registers.iter_mut().enumerate() {
                let accessor =
                    Accessor::new(bus, device, Function::zero(), RegisterIndex::new(i as u8));
                *register = accessor.read();
            }

            Some(Self(registers))
        }
    }

    fn exist(bus: Bus, device: Device) -> bool {
        let accessor = Accessor::new(bus, device, Function::zero(), RegisterIndex::new(0));
        let ids = accessor.read();

        ids != !0
    }
}
impl Index<RegisterIndex> for Registers {
    type Output = u32;

    fn index(&self, index: RegisterIndex) -> &Self::Output {
        &self.0[index.as_u8() as usize]
    }
}
