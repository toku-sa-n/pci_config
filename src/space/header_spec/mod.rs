//! Header type specific fields.
//!
//! There are 3 header types: Standard, PCI-to-PCI bridge, and PCI-to-CardBus bridge.

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
    pub(crate) fn iter_extended_capabilities(
        &self,
    ) -> Option<impl Iterator<Item = Register<'a>> + 'a> {
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
