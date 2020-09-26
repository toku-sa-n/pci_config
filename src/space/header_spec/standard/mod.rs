// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod id;

use {
    super::{
        bar::{self, Bar},
        capability::Pointer,
        InterruptLine, InterruptPin,
    },
    crate::space::registers::Registers,
    id::Id,
};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub(crate) struct HeaderSpecStandard {
    bars: [Bar; 6],
    cardbus_cis_pointer: CardbusCisPointer,
    subsystem_id: Id,
    expansion_rom_base_address: ExpansionRomBaseAddress,
    capabilities_pointer: Pointer,
    interrupt_line: InterruptLine,
    interrupt_pin: InterruptPin,
    min_grant: MinGrant,
    max_latency: MaxLatency,
}

impl HeaderSpecStandard {
    pub(crate) fn parse_registers(registers: &Registers) -> Self {
        let bars = Self::parse_bars(registers);
        let cardbus_cis_pointer = CardbusCisPointer::parse_registers(registers);
        let subsystem_id = Id::parse_registers(registers);
        let expansion_rom_base_address = ExpansionRomBaseAddress::parse_registers(registers);
        let capabilities_pointer = Pointer::parse_registers(registers);
        let interrupt_line = InterruptLine::parse_registers(registers);
        let interrupt_pin = InterruptPin::parse_registers(registers);
        let min_grant = MinGrant::parse_registers(registers);
        let max_latency = MaxLatency::parse_registers(registers);

        Self {
            bars,
            cardbus_cis_pointer,
            subsystem_id,
            expansion_rom_base_address,
            capabilities_pointer,
            interrupt_line,
            interrupt_pin,
            min_grant,
            max_latency,
        }
    }

    fn parse_bars(registers: &Registers) -> [Bar; 6] {
        let mut bars = [Bar::default(); 6];
        for (i, bar) in bars.iter_mut().enumerate() {
            *bar = Bar::parse_registers(registers, bar::Index::new(i));
        }

        bars
    }
}

define_field!(CardbusCisPointer, u32, 0x0a, 0, 0xffff_ffff);
define_field!(ExpansionRomBaseAddress, u32, 0x0c, 0, 0xffff_ffff);
define_field!(MinGrant, u8, 0x0f, 16, 0xff);
define_field!(MaxLatency, u8, 0x0f, 24, 0xff);
