// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

define_field!(SubsystemVendorId, u16, 0x0b, 0, 0xffff);
define_field!(SubsystemId, u16, 0x0b, 16, 0xffff);
