// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A module which defines types for representing PCI configuration space.
macro_rules! define_field {
    ($name:ident,$ty:ty,$index:expr,$shift:expr,$mask:expr) => {
        #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
        pub(crate) struct $name($ty);
        impl $name {
            pub(crate) fn new(registers: &crate::space::registers::Registers) -> Self {
                Self(
                    ((registers[crate::space::accessor::RegisterIndex::new($index)] >> $shift)
                        & $mask) as $ty,
                )
            }
        }
    };
}

pub(crate) mod accessor;
mod common;
mod header_spec;
pub(crate) mod registers;

use {
    common::{header::Header, Common},
    header_spec::capability::register::Register,
    header_spec::HeaderSpec,
    registers::Registers,
};

/// A struct containing information of PCI configuration space.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Space {
    registers: Registers,
}

impl Space {
    /// Returns an iterator over extended capabilities.
    ///
    /// Returning an `Some` value means the PCI configuration space has an capability pointer.
    /// Returning an `None` value means the PCI configuration space doesn't have capability
    /// pointers.
    pub fn iter_extended_capabilities<'a>(&'a self) -> Option<impl Iterator<Item = Register> + 'a> {
        self.header_spec().iter_extended_capabilities()
    }

    pub(crate) fn new(registers: Registers) -> Self {
        Self { registers }
    }

    fn common(&self) -> Common {
        Common::new(&self.registers)
    }

    fn header_spec(&self) -> HeaderSpec {
        HeaderSpec::new(&self.registers, self.header_type())
    }

    fn header_type(&self) -> Header {
        self.common().header_type()
    }

    fn capability_pointer_exists(&self) -> bool {
        self.common().capability_pointer_exists()
    }
}
