// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod class;
mod id;

#[derive(Copy, Clone)]
struct Command(u16);
impl Command {
    fn new(command: u16) -> Self {
        Self(command)
    }
}

#[derive(Copy, Clone)]
struct Status(u16);
impl Status {
    fn new(status: u16) -> Self {
        Self(status)
    }
}

#[derive(Copy, Clone)]
struct Interface(u16);
impl Interface {
    fn new(interface: u16) -> Self {
        Self(interface)
    }
}
