// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub(crate) struct Id {
    subsystem_vendor_id: SubsystemVendorId,
    subsystem_id: SubsystemId,
}

impl Id {
    fn new(subsystem_vendor_id: SubsystemVendorId, subsystem_id: SubsystemId) -> Self {
        Self {
            subsystem_vendor_id,
            subsystem_id,
        }
    }
}

define_field!(SubsystemVendorId, u16, 0x0b, 0, 0xffff);
define_field!(SubsystemId, u16, 0x0b, 16, 0xffff);
