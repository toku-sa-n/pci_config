// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::space::{accessor::RegisterIndex, registers::Registers};

define_field!(Pointer, u8, 0x0d, 0, 0xff);

struct CapabilityRegister {
    common: Common,
}

struct Common {
    id: Id,
    next_pointer: NextPointer,
}
impl Common {
    fn parse_registers(registers: &Registers, base: RegisterIndex) -> Self {
        let id = Id::parse_registers(registers, base);
        let next_pointer = NextPointer::parse_registers(registers, base);

        Self { id, next_pointer }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
struct Id(u8);
impl Id {
    fn parse_registers(registers: &Registers, base: RegisterIndex) -> Self {
        Self((registers[base] & 0xff) as u8)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
struct NextPointer(RegisterIndex);
impl NextPointer {
    fn parse_registers(registers: &Registers, base: RegisterIndex) -> Self {
        Self(RegisterIndex::new(((registers[base] >> 8) & 0xff) as u8))
    }
}
