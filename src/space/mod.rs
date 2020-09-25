// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

macro_rules! define_field {
    ($name:ident,$ty:ty) => {
        #[derive(Copy, Clone)]
        struct $name($ty);
        impl $name {
            fn new(val: $ty) -> Self {
                Self(val)
            }
        }
    };
}

pub(crate) mod accessor;
mod common;
mod registers;
