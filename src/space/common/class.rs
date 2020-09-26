// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::space::registers::Registers;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub(crate) struct Class {
    code: Code,
    sub: Sub,
}
impl Class {
    pub(crate) fn new(registers: &Registers) -> Self {
        let code = Code::new(registers);
        let sub = Sub::new(registers);

        Self { code, sub }
    }
}

define_field!(Code, u8, 2, 24, 0xff);
define_field!(Sub, u8, 2, 16, 0xff);
