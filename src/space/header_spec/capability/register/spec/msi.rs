// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Fields of a capability register which is MSI-specific.

use {
    super::MessageControl,
    crate::space::{accessor::RegisterIndex, registers::Registers},
};

/// A struct which handles MSI-specific fields of a capability register.
pub struct TypeSpecMsi<'a> {
    registers: &'a Registers,
    base: RegisterIndex,
}
impl<'a> TypeSpecMsi<'a> {
    /// Returns a struct which handles message address field of a capability register.
    pub fn message_address(&self) -> MessageAddress {
        MessageAddress::new(self.registers, self.base)
    }

    pub(crate) fn new(registers: &'a Registers, base: RegisterIndex) -> Self {
        Self { registers, base }
    }
}

/// A struct which handles message address field of a capability register.
#[derive(Copy, Clone)]
pub struct MessageAddress<'a> {
    registers: &'a Registers,
    base: RegisterIndex,
}

impl<'a> MessageAddress<'a> {
    /// Get the value of message address.
    pub fn get(&self) -> u64 {
        let lower = self.registers.get(self.base + 1) as u64;
        let upper = self.registers.get(self.base + 2) as u64;

        upper << 32 | lower
    }

    fn new(registers: &'a Registers, base: RegisterIndex) -> Self {
        Self { registers, base }
    }
}

#[derive(Copy, Clone)]
struct MessageData(u16);
impl MessageData {
    fn new(registers: &Registers, base: RegisterIndex) -> Self {
        Self((registers.get(base + 3) & 0xffff) as _)
    }
}
