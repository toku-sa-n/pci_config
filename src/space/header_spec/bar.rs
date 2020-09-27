// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A module containing data structures of Base Address Registers (BAR).
use crate::space::{accessor::RegisterIndex, registers::Registers};

/// Base Address Registers (BAR).
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct Bar(u32);
impl Bar {
    pub(crate) fn new(registers: &Registers, index: Index) -> Self {
        Self(registers.get(RegisterIndex::new(4 + index.as_usize() as u8)))
    }
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
