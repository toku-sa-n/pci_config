// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod class;
pub(crate) mod header;
mod id;

use {
    crate::space::registers::Registers,
    class::Class,
    header::{Header, HeaderType},
    id::Id,
};

/// The fields of PCI headers all PCI devices have.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Common<'a> {
    registers: &'a Registers,
}
impl<'a> Common<'a> {
    pub(crate) fn new(registers: &'a Registers) -> Self {
        // let id = Id::new(registers);
        // let command = Command::new(registers);
        // let status = Status::new(registers);
        // let interface = Interface::new(registers);
        // let class = Class::new(registers);
        // let cache_line_size = CacheLineSize::new(registers);
        // let latency_timer = LatencyTimer::new(registers);
        // let header_type = HeaderType::new(registers);
        // let bist = Bist::new(registers);

        Self { registers }
    }

    pub(crate) fn header_type(&self) -> Header {
        let header_type = HeaderType::new(self.registers);
        header_type.ty()
    }

    pub(crate) fn capability_pointer_exists(&self) -> bool {
        self.status().capability_pointer_exists()
    }

    fn status(&self) -> Status {
        Status::new(self.registers)
    }
}

define_field!(Command, u16, 1, 0, 0xffff);
define_field!(Status, u16, 1, 16, 0xffff);
define_field!(Interface, u8, 2, 8, 0xff);
define_field!(CacheLineSize, u8, 3, 0, 0xff);
define_field!(LatencyTimer, u8, 3, 8, 0xff);
define_field!(Bist, u8, 3, 24, 0xff);

impl Status {
    fn capability_pointer_exists(self) -> bool {
        self.0 & 0b1_0000 != 0
    }
}
