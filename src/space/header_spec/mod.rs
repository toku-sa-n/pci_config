// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod bar;
pub mod capability;
mod standard;

use {
    super::{common::header::Header, registers::Registers},
    crate::space::header_spec::capability::register::Register,
    standard::HeaderSpecStandard,
};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub(crate) enum HeaderSpec<'a> {
    Standard(HeaderSpecStandard<'a>),
}

impl<'a> HeaderSpec<'a> {
    pub(crate) fn iter_extended_capabilities(&self) -> Option<impl Iterator<Item = Register> + 'a> {
        match self {
            Self::Standard(standard) => Some(standard.iter_extended_capabilities()),
            _ => None,
        }
    }

    pub(crate) fn new(registers: &'a Registers, header: Header) -> Self {
        match header {
            Header::Standard => Self::Standard(HeaderSpecStandard::new(registers)),
            _ => todo!(),
        }
    }
}

define_field!(InterruptLine, u8, 0x0f, 0, 0xff);
define_field!(InterruptPin, u8, 0x0f, 8, 0xff);
