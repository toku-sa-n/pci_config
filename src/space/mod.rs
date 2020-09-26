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

use {common::Common, header_spec::HeaderSpec, registers::Registers};

/// A struct containing information of PCI configuration space.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Space {
    common: Common,
    header_spec: HeaderSpec,
}

impl Space {
    pub(crate) fn new(registers: &Registers) -> Self {
        let common = Common::new(registers);
        let header_spec = HeaderSpec::new(registers, common.header_type());

        Self {
            common,
            header_spec,
        }
    }
}
