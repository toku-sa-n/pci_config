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
struct Interface(u8);
impl Interface {
    fn new(interface: u8) -> Self {
        Self(interface)
    }
}

#[derive(Copy, Clone)]
struct CacheLineSize(u8);
impl CacheLineSize {
    fn new(size: u8) -> Self {
        Self(size)
    }
}

#[derive(Copy, Clone)]
struct LatencyTimer(u8);
impl LatencyTimer {
    fn new(timer: u8) -> Self {
        Self(timer)
    }
}
