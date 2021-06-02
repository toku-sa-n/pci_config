pub mod register;

use {
    crate::space::{accessor::RegisterIndex, registers::Registers},
    core::iter::Iterator,
    register::Register,
};

define_field!(Pointer, u8, 0x0d, 0, 0xfc);
impl Pointer {
    pub(crate) fn iter_registers<'a>(
        self,
        registers: &'a Registers,
    ) -> impl Iterator<Item = Register> + 'a {
        IterRegisters::new(registers, RegisterIndex::new(self.0 / 4))
    }
}

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
    type Item = Register<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next.is_zero() {
            None
        } else {
            let register = Register::new(self.registers, self.next);
            self.next = register.next_index();

            Some(register)
        }
    }
}
