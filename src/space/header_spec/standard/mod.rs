// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod id;

use {
    super::{
        bar::{self, Bar},
        capability::register::Register,
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
    pub(crate) fn new(registers: &Registers) -> Self {
        let bars = Self::parse_bars(registers);
        let cardbus_cis_pointer = CardbusCisPointer::new(registers);
        let subsystem_id = Id::new(registers);
        let expansion_rom_base_address = ExpansionRomBaseAddress::new(registers);
        let capabilities_pointer = Pointer::new(registers);
        let interrupt_line = InterruptLine::new(registers);
        let interrupt_pin = InterruptPin::new(registers);
        let min_grant = MinGrant::new(registers);
        let max_latency = MaxLatency::new(registers);

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
            *bar = Bar::new(registers, bar::Index::new(i));
        }

        bars
    }

    fn iter_extended_capabilities<'a>(
        &self,
        registers: &'a Registers,
    ) -> impl Iterator<Item = Register> + 'a {
        self.capabilities_pointer.iter_registers(registers)
    }
}

define_field!(CardbusCisPointer, u32, 0x0a, 0, 0xffff_ffff);
define_field!(ExpansionRomBaseAddress, u32, 0x0c, 0, 0xffff_ffff);
define_field!(MinGrant, u8, 0x0f, 16, 0xff);
define_field!(MaxLatency, u8, 0x0f, 24, 0xff);
