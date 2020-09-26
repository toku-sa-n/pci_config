// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::MessageControl;

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
}

#[derive(Copy, Clone)]
struct MessageData(u16);
impl MessageData {
    fn new(data: u16) -> Self {
        Self(data)
    }
}
