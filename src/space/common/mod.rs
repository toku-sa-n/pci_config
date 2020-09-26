// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod class;
mod id;

use {class::Class, id::Id};

struct Common {
    id: Id,
    command: Command,
    status: Status,
    interface: Interface,
    class: Class,
    cache_line_size: CacheLineSize,
    latency_timer: LatencyTimer,
    header_type: HeaderType,
    bist: Bist,
}
impl Common {
    fn new(
        id: Id,
        command: Command,
        status: Status,
        interface: Interface,
        class: Class,
        cache_line_size: CacheLineSize,
        latency_timer: LatencyTimer,
        header_type: HeaderType,
        bist: Bist,
    ) -> Self {
        Self {
            id,
            command,
            status,
            interface,
            class,
            cache_line_size,
            latency_timer,
            header_type,
            bist,
        }
    }
}

define_field!(Command, u16, 1, 0, 0xffff);
define_field!(Status, u16, 1, 16, 0xffff);
define_field!(Interface, u8, 2, 8, 0xff);
define_field!(CacheLineSize, u8, 3, 0, 0xff);
define_field!(LatencyTimer, u8, 3, 8, 0xff);
define_field!(HeaderType, u8, 3, 16, 0xff);
define_field!(Bist, u8, 3, 24, 0xff);
