// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use {
    crate::space::header_spec::bar,
    os_units::{Bytes, Size},
};

struct Bir(bar::Index);
impl Bir {
    fn new(bir: bar::Index) -> Self {
        Self(bir)
    }
}

struct TableOffset(Size<Bytes>);
