// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

macro_rules! define_field {
    ($name:ident,$ty:ty,$index:expr,$shift:expr,$mask:expr) => {
        #[derive(Copy, Clone)]
        struct $name($ty);
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
mod registers;

use {common::Common, header_spec::HeaderSpec};

struct Space {
    common: Common,
    header_spec: HeaderSpec,
}

impl Space {
    fn new(common: Common, header_spec: HeaderSpec) -> Self {
        Self {
            common,
            header_spec,
        }
    }
}
