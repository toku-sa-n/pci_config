//! A module containing data structures of Base Address Registers (BAR).
use crate::space::{accessor::RegisterIndex, registers::Registers};

/// Base Address Registers (BAR).
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct Bar(u32);
impl Bar {
    pub(crate) fn new(registers: &Registers, index: Index) -> Self {
        Self(registers.get(RegisterIndex::new(4 + index.as_usize() as u8)))
    }

    pub(crate) fn base_address(self, upper: Option<Self>) -> Option<u64> {
        if self.address_length() == AddressLength::Bit32 {
            Some(self.address_32())
        } else if upper.is_some() {
            Some(self.address_64(upper.unwrap()))
        } else {
            None
        }
    }

    fn address_32(self) -> u64 {
        (self.0 & !0xf) as _
    }

    fn address_64(self, upper: Self) -> u64 {
        (upper.0 as u64) << 32 | self.address_32()
    }

    fn address_length(self) -> AddressLength {
        match (self.0 >> 1) & 0b11 {
            0 => AddressLength::Bit32,
            2 => AddressLength::Bit64,
            1 => unimplemented!(),
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum AddressLength {
    Bit32,
    Bit64,
}

/// A structure used to specify which BAR to use.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct Index(usize);
impl Index {
    /// Create an instance of this type.
    ///
    /// # Panics
    ///
    /// Panics if `index >= 6` as PCI configuration space never has more than 5 BARs.
    pub fn new(index: usize) -> Self {
        assert!(index < 6);
        Self(index)
    }

    fn as_usize(self) -> usize {
        self.0
    }
}
