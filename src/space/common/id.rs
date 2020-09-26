// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::space::registers::Registers;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub(crate) struct Id {
    device: Device,
    vendor: Vendor,
    revision: Revision,
}

impl Id {
    pub(crate) fn new(registers: &Registers) -> Self {
        let device = Device::new(registers);
        let vendor = Vendor::new(registers);
        let revision = Revision::new(registers);

        Self {
            device,
            vendor,
            revision,
        }
    }
}

define_field!(Device, u16, 0, 16, 0xffff);
define_field!(Vendor, u16, 0, 0, 0xffff);
define_field!(Revision, u8, 2, 0, 0xff);
