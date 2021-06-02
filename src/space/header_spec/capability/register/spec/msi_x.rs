//! Fields of a capability register which is MSI-X specific.

/// A module containing codes about MSI-X.
use {
    super::MessageControl,
    crate::space::{accessor::RegisterIndex, header_spec::bar, registers::Registers},
    os_units::Bytes,
};

/// A structure which handles MSI-X specific fields.
pub struct TypeSpecMsiX<'a> {
    registers: &'a Registers,
    base: RegisterIndex,
}
impl<'a> TypeSpecMsiX<'a> {
    /// Returns a BIR value which specifies a BAR for a message table.
    pub fn bir(&self) -> bar::Index {
        Bir::new(self.registers, self.base).as_bar_index()
    }

    /// Returns an offset from the BAR specified by BIR, where a message table exists.
    pub fn table_offset(&self) -> usize {
        TableOffset::new(self.registers, self.base).as_usize()
    }

    pub(crate) fn new(registers: &'a Registers, base: RegisterIndex) -> Self {
        Self { registers, base }
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

struct TableOffset(Bytes);
impl TableOffset {
    fn new(registers: &Registers, base: RegisterIndex) -> Self {
        let offset = registers.get(base + 1) & !0b111;
        assert!(offset.trailing_zeros() >= 2);
        Self(Bytes::new(offset as _))
    }

    fn as_usize(&self) -> usize {
        self.0.as_usize()
    }
}

struct PendingBitBir(bar::Index);
impl PendingBitBir {
    fn new(registers: &Registers, base: RegisterIndex) -> Self {
        Self(bar::Index::new((registers.get(base + 2) & !0b111) as _))
    }
}

struct PendingBitOffset(Bytes);
impl PendingBitOffset {
    fn new(registers: &Registers, base: RegisterIndex) -> Self {
        let offset = registers.get(base + 2) & !0b111;
        assert!(offset.trailing_zeros() >= 2);
        Self(Bytes::new(offset as _))
    }
}
