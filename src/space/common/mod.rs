// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod class;
mod id;

define_field!(Command, u16);
define_field!(Status, u16);
define_field!(Interface, u8);
define_field!(CacheLineSize, u8);
define_field!(LatencyTimer, u8);
define_field!(HeaderType, u8);
define_field!(Bist, u8);
