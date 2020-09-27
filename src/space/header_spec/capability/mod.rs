// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod register;

use {
    crate::space::{accessor::RegisterIndex, registers::Registers},
    core::iter::Iterator,
    register::Register,
};

define_field!(Pointer, u8, 0x0d, 0, 0xfc);
impl Pointer {}

struct IterRegisters<'a> {
    registers: &'a Registers,
    next: RegisterIndex,
}
impl<'a> IterRegisters<'a> {
    fn new(registers: &'a Registers, capability_pointer: RegisterIndex) -> Self {
        Self {
            next: capability_pointer,
            registers,
        }
    }
}
impl<'a> Iterator for IterRegisters<'a> {
    type Item = Register;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next.is_zero() {
            None
        } else {
            let register = Register::parse_registers(self.registers, self.next);
            self.next = register.next_index();

            Some(register)
        }
    }
}
