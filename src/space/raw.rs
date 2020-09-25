// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::accessor::{Accessor, Bus, Device, Function, RegisterIndex};

pub(crate) const NUM_REGISTERS: usize = 64;

struct Registers([u32; NUM_REGISTERS]);
impl Registers {
    fn exist(bus: Bus, device: Device) -> bool {
        let accessor = Accessor::new(bus, device, Function::zero(), RegisterIndex::new(0));
        let ids = accessor.read();

        ids != !0
    }
}
