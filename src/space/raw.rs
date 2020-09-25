// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::accessor::{Accessor, Bus, Device, Function, RegisterIndex};

struct Registers([u32; RegisterIndex::MAX]);
impl Registers {
    fn fetch(bus: Bus, device: Device) -> Option<Self> {
        if !Self::exist(bus, device) {
            None
        } else {
            let mut registers = [0u32; RegisterIndex::MAX];

            for i in 0..RegisterIndex::MAX {
                let accessor =
                    Accessor::new(bus, device, Function::zero(), RegisterIndex::new(i as u8));
                registers[i] = accessor.read();
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
