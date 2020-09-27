// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Fields of a capability register which is MSI-specific.

use {
    super::MessageControl,
    crate::space::{accessor::RegisterIndex, registers::Registers},
};

pub struct TypeSpecMsi<'a> {
    registers: &'a Registers,
    base: RegisterIndex,
}
impl<'a> TypeSpecMsi<'a> {
    pub(crate) fn new(registers: &'a Registers, base: RegisterIndex) -> Self {
        Self { registers, base }
    }
}

#[derive(Copy, Clone)]
struct MessageAddress(u64);
impl MessageAddress {
    fn new(registers: &Registers, base: RegisterIndex) -> Self {
        let lower = registers.get(base + 1) as u64;
        let upper = registers.get(base + 2) as u64;

        Self(upper << 32 | lower)
    }
}

#[derive(Copy, Clone)]
struct MessageData(u16);
impl MessageData {
    fn new(registers: &Registers, base: RegisterIndex) -> Self {
        Self((registers.get(base + 3) & 0xffff) as _)
    }
}
