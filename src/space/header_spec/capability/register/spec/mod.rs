// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod msi;
mod msi_x;

use crate::space::{accessor::RegisterIndex, registers::Registers};

#[derive(Copy, Clone)]
struct MessageControl(u16);
impl MessageControl {
    fn new(control: u16) -> Self {
        Self(control)
    }

    fn parse_registers(registers: &Registers, base: RegisterIndex) -> Self {
        Self::new(((registers[base] >> 16) & 0xffff) as _)
    }
}
