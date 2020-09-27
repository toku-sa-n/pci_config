// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Fields which all capability registers have.
use crate::space::{accessor::RegisterIndex, registers::Registers};

/// A structure providing information values which all capability registers have.
pub(crate) struct Common {
    id: Id,
    next_pointer: NextPointer,
}
impl Common {
    pub(crate) fn new(registers: &Registers, base: RegisterIndex) -> Self {
        let id = Id::parse_registers(registers, base);
        let next_pointer = NextPointer::parse_registers(registers, base);

        Self { id, next_pointer }
    }

    pub(crate) fn next_index(&self) -> RegisterIndex {
        self.next_pointer.as_register_index()
    }

    pub(crate) fn ty(&self) -> Option<Type> {
        self.id.ty()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
struct Id(u8);
impl Id {
    fn parse_registers(registers: &Registers, base: RegisterIndex) -> Self {
        Self((registers.get(base) & 0xff) as u8)
    }

    fn ty(self) -> Option<Type> {
        match self.0 {
            0x05 => Some(Type::Msi),
            0x11 => Some(Type::MsiX),
            _ => None,
        }
    }
}

pub enum Type {
    Msi,
    MsiX,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
struct NextPointer(RegisterIndex);
impl NextPointer {
    fn parse_registers(registers: &Registers, base: RegisterIndex) -> Self {
        Self(RegisterIndex::new(
            ((registers.get(base) >> 8) & 0xff) as u8,
        ))
    }

    fn as_register_index(&self) -> RegisterIndex {
        self.0
    }
}
