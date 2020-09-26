// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

struct Class {
    code: Code,
    sub: Sub,
}
impl Class {
    fn new(code: Code, sub: Sub) -> Self {
        Self { code, sub }
    }
}

define_field!(Code, u8, 2, 24, 0xff);
define_field!(Sub, u8, 2, 16, 0xff);
