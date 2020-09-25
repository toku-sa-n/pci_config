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

#[derive(Copy, Clone)]
struct Device(u16);
impl Device {
    fn new(device: u16) -> Self {
        Self(device)
    }
}

#[derive(Copy, Clone)]
struct Vendor(u16);
impl Vendor {
    fn new(vendor: u16) -> Self {
        Self(vendor)
    }
}

#[derive(Copy, Clone)]
struct Revision(u8);
impl Revision {
    fn new(revision: u8) -> Self {
        Self(revision)
    }
}
