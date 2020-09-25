// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

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
