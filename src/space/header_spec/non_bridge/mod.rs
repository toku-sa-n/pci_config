// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod id;

use {
    super::{bar::Bar, CapabilitiesPointer, InterruptLine, InterruptPin},
    id::Id,
};

pub(crate) struct HeaderSpecNonBridge {
    bars: [Bar; 6],
    cardbus_cis_pointer: CardbusCisPointer,
    subsystem_id: Id,
    expansion_rom_base_address: ExpansionRomBaseAddress,
    capabilities_pointer: CapabilitiesPointer,
    interrupt_line: InterruptLine,
    interrupt_pin: InterruptPin,
    min_grant: MinGrant,
    max_latency: MaxLatency,
}

define_field!(CardbusCisPointer, u32, 0x0a, 0, 0xffff_ffff);
define_field!(ExpansionRomBaseAddress, u32, 0x0c, 0, 0xffff_ffff);
define_field!(MinGrant, u8, 0x0f, 16, 0xff);
define_field!(MaxLatency, u8, 0x0f, 24, 0xff);
