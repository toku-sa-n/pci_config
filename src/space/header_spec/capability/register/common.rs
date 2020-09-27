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
    /// Returns the type of this capability register.
    pub fn ty(&self) -> Result<Type, Error> {
        self.id.ty()
    }

    pub(crate) fn new(registers: &Registers, base: RegisterIndex) -> Self {
        let id = Id::parse_registers(registers, base);
        let next_pointer = NextPointer::parse_registers(registers, base);

        Self { id, next_pointer }
    }

    pub(crate) fn next_index(&self) -> RegisterIndex {
        self.next_pointer.as_register_index()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
struct Id(u8);
impl Id {
    fn parse_registers(registers: &Registers, base: RegisterIndex) -> Self {
        Self((registers.get(base) & 0xff) as u8)
    }

    fn ty(self) -> Result<Type, Error> {
        match self.0 {
            0x05 => Ok(Type::Msi),
            0x11 => Ok(Type::MsiX),
            e if e <= 0x15 => Err(Error::NotYetSupported(self.0)),
            _ => Err(Error::ReservedId(self.0)),
        }
    }
}

/// The type of a capability register which is currently supported by this crate.
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

/// An enum representing errors.
///
/// Each enum variant has an ID of a capability register.
pub enum Error {
    /// Currently this type of capability registers are not supported by this crate.
    NotYetSupported(u8),
    /// This ID is reserved.
    ReservedId(u8),
}
