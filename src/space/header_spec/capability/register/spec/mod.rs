// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod msi;
mod msi_x;

use {
    super::common::{Common, Type},
    crate::space::{accessor::RegisterIndex, registers::Registers},
    msi::TypeSpecMsi,
    msi_x::TypeSpecMsiX,
};

pub(crate) enum TypeSpec {
    Msi(TypeSpecMsi),
    MsiX(TypeSpecMsiX),
}
impl TypeSpec {
    pub(crate) fn new(registers: &Registers, base: RegisterIndex, common: &Common) -> Option<Self> {
        match common.ty() {
            None => None,
            Some(ty) => Some(match ty {
                Type::Msi => Self::Msi(TypeSpecMsi::new(registers, base)),
                Type::MsiX => Self::MsiX(TypeSpecMsiX::parse_registers(registers, base)),
            }),
        }
    }
}

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
