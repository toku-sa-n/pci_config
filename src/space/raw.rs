// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::accessor::{Bus, Device, RegisterIndex};

pub(crate) const NUM_REGISTERS: usize = 64;

struct Registers([u32; NUM_REGISTERS]);