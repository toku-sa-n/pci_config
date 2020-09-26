// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

define_field!(HeaderType, u8, 3, 16, 0xff);
impl HeaderType {
    fn ty(self) -> Header {
        match self.0 & 0x7f {
            0 => Header::Standard,
            1 => Header::PciToPciBridge,
            2 => Header::CardBusBridge,
            _ => unreachable!(),
        }
    }
}

enum Header {
    Standard,
    PciToPciBridge,
    CardBusBridge,
}
