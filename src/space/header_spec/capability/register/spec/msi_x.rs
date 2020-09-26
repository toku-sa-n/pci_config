// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use {
    super::MessageControl,
    crate::space::header_spec::bar,
    os_units::{Bytes, Size},
};

struct TypeSpecMsiX {
    message_control: MessageControl,
    bir: Bir,
    table_offset: TableOffset,
    pending_bit_bir: PendingBitBir,
    pending_bit_offset: PendingBitOffset,
}

struct Bir(bar::Index);
impl Bir {
    fn new(bir: bar::Index) -> Self {
        Self(bir)
    }
}

struct TableOffset(Size<Bytes>);
impl TableOffset {
    fn new(offset: u32) -> Self {
        assert!(offset.trailing_zeros() >= 2);
        Self(Size::new(offset as _))
    }
}

struct PendingBitBir(bar::Index);
impl PendingBitBir {
    fn new(bir: bar::Index) -> Self {
        Self(bir)
    }
}

struct PendingBitOffset(Size<Bytes>);
impl PendingBitOffset {
    fn new(offset: u32) -> Self {
        assert!(offset.trailing_zeros() >= 2);
        Self(Size::new(offset as _))
    }
}
