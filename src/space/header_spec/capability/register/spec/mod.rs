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

pub(crate) enum TypeSpec<'a> {
    Msi(TypeSpecMsi<'a>),
    MsiX(TypeSpecMsiX<'a>),
}
impl<'a> TypeSpec<'a> {
    pub(crate) fn new(
        registers: &'a Registers,
        base: RegisterIndex,
        common: &Common,
    ) -> Option<Self> {
        match common.ty() {
            None => None,
            Some(ty) => Some(match ty {
                Type::Msi => Self::Msi(TypeSpecMsi::new(registers, base)),
                Type::MsiX => Self::MsiX(TypeSpecMsiX::new(registers, base)),
            }),
        }
    }
}

#[derive(Copy, Clone)]
struct MessageControl(u16);
impl MessageControl {
    fn new(registers: &Registers, base: RegisterIndex) -> Self {
        Self(((registers[base] >> 16) & 0xffff) as _)
    }
}
