// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::super::{accessor::RegisterIndex, registers::Registers};

struct Id {
    device: Device,
    vendor: Vendor,
    revision: Revision,
}

impl Id {
    fn new(device: Device, vendor: Vendor, revision: Revision) -> Self {
        Self {
            device,
            vendor,
            revision,
        }
    }
}

define_field!(Device, u16);
impl Device {
    fn parse_registers(registers: &Registers) -> Self {
        Self::new(((registers[RegisterIndex::new(0)] >> 16) & 0xffff) as _)
    }
}
define_field!(Vendor, u16);
define_field!(Revision, u8);
