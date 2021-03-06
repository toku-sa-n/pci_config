//! Fields which not all capability registers have.

pub mod msi;
pub mod msi_x;

use {
    super::common::{Common, Type},
    crate::space::{accessor::RegisterIndex, registers::Registers},
    msi::TypeSpecMsi,
    msi_x::TypeSpecMsiX,
};

/// An enum containing a struct which handles register-specific fields.
pub enum TypeSpec<'a> {
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
            Err(_) => None,
            Ok(ty) => Some(match ty {
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
        Self(((registers.get(base) >> 16) & 0xffff) as _)
    }
}
