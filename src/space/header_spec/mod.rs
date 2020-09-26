// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod bar;
mod non_bridge;

use non_bridge::HeaderSpecNonBridge;

enum HeaderSpec {
    NonBridge(HeaderSpecNonBridge),
}

define_field!(CapabilitiesPointer, u8, 0x0d, 0, 0xff);
define_field!(InterruptLine, u8, 0x0f, 0, 0xff);
define_field!(InterruptPin, u8, 0x0f, 8, 0xff);
