// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod common;

use {
    crate::space::{accessor::RegisterIndex, registers::Registers},
    common::Common,
};

define_field!(Pointer, u8, 0x0d, 0, 0xff);

struct Register {
    common: Common,
}
impl Register {
    fn parse_registers(registers: &Registers, base: RegisterIndex) -> Self {
        let common = Common::parse_registers(registers, base);

        Self { common }
    }
}
