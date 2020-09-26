// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::space::registers::Registers;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub(crate) struct Id {
    subsystem_vendor_id: SubsystemVendorId,
    subsystem_id: SubsystemId,
}

impl Id {
    pub(crate) fn parse_registers(registers: &Registers) -> Self {
        let subsystem_vendor_id = SubsystemVendorId::parse_registers(registers);
        let subsystem_id = SubsystemId::parse_registers(registers);

        Self::new(subsystem_vendor_id, subsystem_id)
    }

    fn new(subsystem_vendor_id: SubsystemVendorId, subsystem_id: SubsystemId) -> Self {
        Self {
            subsystem_vendor_id,
            subsystem_id,
        }
    }
}

define_field!(SubsystemVendorId, u16, 0x0b, 0, 0xffff);
define_field!(SubsystemId, u16, 0x0b, 16, 0xffff);
