// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use {
    super::MessageControl,
    crate::space::{accessor::RegisterIndex, registers::Registers},
};

pub(crate) struct TypeSpecMsi {
    message_control: MessageControl,
    message_address: MessageAddress,
    message_data: MessageData,
}
impl TypeSpecMsi {
    pub(crate) fn new(registers: &Registers, base: RegisterIndex) -> Self {
        let message_control = MessageControl::new(registers, base);
        let message_address = MessageAddress::new(registers, base);
        let message_data = MessageData::new(registers, base);

        Self {
            message_control,
            message_address,
            message_data,
        }
    }
}

#[derive(Copy, Clone)]
struct MessageAddress(u64);
impl MessageAddress {
    fn new(registers: &Registers, base: RegisterIndex) -> Self {
        let lower = registers[base + 1] as u64;
        let upper = registers[base + 2] as u64;

        Self(upper << 32 | lower)
    }
}

#[derive(Copy, Clone)]
struct MessageData(u16);
impl MessageData {
    fn new(registers: &Registers, base: RegisterIndex) -> Self {
        Self((registers[base + 3] & 0xffff) as _)
    }
}
