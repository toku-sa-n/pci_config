// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use {
    super::MessageControl,
    crate::space::{accessor::RegisterIndex, header_spec::bar, registers::Registers},
    os_units::{Bytes, Size},
};

pub(crate) struct TypeSpecMsiX {
    message_control: MessageControl,
    bir: Bir,
    table_offset: TableOffset,
    pending_bit_bir: PendingBitBir,
    pending_bit_offset: PendingBitOffset,
}
impl TypeSpecMsiX {
    pub(crate) fn new(registers: &Registers, base: RegisterIndex) -> Self {
        let message_control = MessageControl::new(registers, base);
        let bir = Bir::new(registers, base);
        let table_offset = TableOffset::new(registers, base);
        let pending_bit_bir = PendingBitBir::new(registers, base);
        let pending_bit_offset = PendingBitOffset::parse_registers(registers, base);

        Self {
            message_control,
            bir,
            table_offset,
            pending_bit_bir,
            pending_bit_offset,
        }
    }
}

struct Bir(bar::Index);
impl Bir {
    fn new(registers: &Registers, base: RegisterIndex) -> Self {
        Self(bar::Index::new((registers[base + 1] & 0b111) as _))
    }
}

struct TableOffset(Size<Bytes>);
impl TableOffset {
    fn new(registers: &Registers, base: RegisterIndex) -> Self {
        let offset = registers[base + 1] & !0b111;
        assert!(offset.trailing_zeros() >= 2);
        Self(Size::new(offset as _))
    }
}

struct PendingBitBir(bar::Index);
impl PendingBitBir {
    fn new(registers: &Registers, base: RegisterIndex) -> Self {
        Self(bar::Index::new((registers[base + 2] & !0b111) as _))
    }
}

struct PendingBitOffset(Size<Bytes>);
impl PendingBitOffset {
    fn new(offset: u32) -> Self {
        assert!(offset.trailing_zeros() >= 2);
        Self(Size::new(offset as _))
    }

    fn parse_registers(registers: &Registers, base: RegisterIndex) -> Self {
        Self::new(registers[base + 2] & !0b111)
    }
}
