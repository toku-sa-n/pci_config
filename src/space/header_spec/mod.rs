// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod bar;
mod capabilities_list;
mod standard;

use {
    super::{common::header::Header, registers::Registers},
    standard::HeaderSpecStandard,
};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub(crate) enum HeaderSpec {
    Standard(HeaderSpecStandard),
}

impl HeaderSpec {
    pub(crate) fn parse_registers(registers: &Registers, header: Header) -> Self {
        match header {
            Header::Standard => Self::Standard(HeaderSpecStandard::parse_registers(registers)),
            _ => todo!(),
        }
    }
}

define_field!(InterruptLine, u8, 0x0f, 0, 0xff);
define_field!(InterruptPin, u8, 0x0f, 8, 0xff);
