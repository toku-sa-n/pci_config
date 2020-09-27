// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Fields of a capability register which is MSI-X specific.

/// A module containing codes about MSI-X.
use {
    super::MessageControl,
    crate::space::{accessor::RegisterIndex, header_spec::bar, registers::Registers},
    os_units::{Bytes, Size},
};

/// A structure which handles MSI-X specific fields.
pub struct TypeSpecMsiX<'a> {
    registers: &'a Registers,
    base: RegisterIndex,
}
impl<'a> TypeSpecMsiX<'a> {
    pub(crate) fn new(registers: &'a Registers, base: RegisterIndex) -> Self {
        Self { registers, base }
    }

    pub fn bir(&self) -> bar::Index {
        Bir::new(self.registers, self.base).as_bar_index()
    }
}

struct Bir(bar::Index);
impl Bir {
    fn new(registers: &Registers, base: RegisterIndex) -> Self {
        Self(bar::Index::new((registers.get(base + 1) & 0b111) as _))
    }

    fn as_bar_index(&self) -> bar::Index {
        self.0
    }
}

struct TableOffset(Size<Bytes>);
impl TableOffset {
    fn new(registers: &Registers, base: RegisterIndex) -> Self {
        let offset = registers.get(base + 1) & !0b111;
        assert!(offset.trailing_zeros() >= 2);
        Self(Size::new(offset as _))
    }
}

struct PendingBitBir(bar::Index);
impl PendingBitBir {
    fn new(registers: &Registers, base: RegisterIndex) -> Self {
        Self(bar::Index::new((registers.get(base + 2) & !0b111) as _))
    }
}

struct PendingBitOffset(Size<Bytes>);
impl PendingBitOffset {
    fn new(registers: &Registers, base: RegisterIndex) -> Self {
        let offset = registers.get(base + 2) & !0b111;
        assert!(offset.trailing_zeros() >= 2);
        Self(Size::new(offset as _))
    }
}
