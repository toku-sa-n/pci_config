// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[derive(Copy, Clone)]
pub(crate) struct Bar(u32);
impl Bar {
    fn new(bar: u32) -> Self {
        Self(bar)
    }
}

#[derive(Copy, Clone)]
struct Index(usize);
impl Index {
    fn new(index: usize) -> Self {
        assert!(index < 6);
        Self(index)
    }
}
