// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use {
    super::MessageControl,
    crate::space::{accessor::RegisterIndex, registers::Registers},
};

struct TypeSpecMsi {
    message_control: MessageControl,
    message_address: MessageAddress,
    message_data: MessageData,
}

#[derive(Copy, Clone)]
struct MessageAddress(u64);
impl MessageAddress {
    fn new(address: u64) -> Self {
        Self(address)
    }

    fn parse_registers(registers: &Registers, base: RegisterIndex) -> Self {
        let lower = registers[base + 1] as u64;
        let upper = registers[base + 2] as u64;

        Self::new(upper << 32 | lower)
    }
}

#[derive(Copy, Clone)]
struct MessageData(u16);
impl MessageData {
    fn new(data: u16) -> Self {
        Self(data)
    }
}
