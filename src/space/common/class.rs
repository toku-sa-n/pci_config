// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

struct Code(u8);
impl Code {
    fn new(code: u8) -> Self {
        Self(code)
    }
}

struct Sub(u8);
impl Sub {
    fn new(sub: u8) -> Self {
        Self(sub)
    }
}
