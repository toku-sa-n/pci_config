// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

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
define_field!(Vendor, u16);
define_field!(Revision, u8);
