// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub(crate) struct Id {
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

define_field!(Device, u16, 0, 16, 0xffff);
define_field!(Vendor, u16, 0, 0, 0xffff);
define_field!(Revision, u8, 2, 0, 0xff);
