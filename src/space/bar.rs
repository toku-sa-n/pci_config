// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[derive(Copy, Clone)]
struct Bar(u32);
impl Bar {
    fn new(bar: u32) -> Self {
        Self(bar)
    }
}
