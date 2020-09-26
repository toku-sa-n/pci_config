// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

macro_rules! define_field {
    ($name:ident,$ty:ty,$index:expr,$shift:expr,$mask:expr) => {
        #[derive(Copy, Clone)]
        pub(crate) struct $name($ty);
        impl $name {
            pub(crate) fn parse_registers(registers: &crate::space::registers::Registers) -> Self {
                Self::new(
                    ((registers[crate::space::accessor::RegisterIndex::new($index)] >> $shift)
                        & $mask) as $ty,
                )
            }

            fn new(val: $ty) -> Self {
                Self(val)
            }
        }
    };
}

pub(crate) mod accessor;
mod common;
mod header_spec;
pub(crate) mod registers;

use {common::Common, header_spec::HeaderSpec, registers::Registers};

pub(crate) struct Space {
    common: Common,
    header_spec: HeaderSpec,
}

impl Space {
    fn parse_registers(registers: &Registers) -> Self {
        let common = Common::parse_registers(registers);
        let header_spec = HeaderSpec::parse_registers(registers, common.header_type());

        Self::new(common, header_spec)
    }

    fn new(common: Common, header_spec: HeaderSpec) -> Self {
        Self {
            common,
            header_spec,
        }
    }
}
