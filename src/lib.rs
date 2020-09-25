// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![no_std]
#![deny(clippy::all)]

mod space;

use space::access::{Bus, Device};

struct IterDevices {
    bus: Bus,
    device: Device,
}

impl IterDevices {
    fn new(bus: Bus, device: Device) -> Self {
        Self { bus, device }
    }
}
