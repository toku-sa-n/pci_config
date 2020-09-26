// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::space::{accessor::RegisterIndex, registers::Registers};

#[derive(Copy, Clone)]
pub(crate) struct Bar(u32);
impl Bar {
    fn parse_registers(registers: &Registers, index: Index) -> Self {
        Self::new(registers[RegisterIndex::new(4 + index.as_usize() as u8)])
    }

    fn new(bar: u32) -> Self {
        Self(bar)
    }
}

#[derive(Copy, Clone)]
struct Index(usize);
impl Index {
    fn new(index: usize) -> Self {
        assert!(index < 6);
        Self(index)
    }

    fn as_usize(self) -> usize {
        self.0
    }
}
