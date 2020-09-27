// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::space::{accessor::RegisterIndex, registers::Registers};

/// Base Address Registers (BAR)

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct Bar(u32);
impl Bar {
    pub(crate) fn new(registers: &Registers, index: Index) -> Self {
        Self(registers.get(RegisterIndex::new(4 + index.as_usize() as u8)))
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct Index(usize);
impl Index {
    pub fn new(index: usize) -> Self {
        assert!(index < 6);
        Self(index)
    }

    fn as_usize(self) -> usize {
        self.0
    }
}
