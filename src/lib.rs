//! This crate provides useful tools for handling PCI devices.

#![no_std]
#![deny(clippy::all)]

pub mod port;
pub mod space;

use {
    core::iter::Iterator,
    space::{
        accessor::{Bus, Device},
        registers::Registers,
        Space,
    },
};

/// Returns an iterator which returns a structure containing information of PCI configuration space.
///
/// This iterator searches all bus numbers and device numbers
pub fn iter_device() -> impl Iterator<Item = Space> {
    IterDevices::new(0, 0)
}

struct IterDevices {
    bus: u8,
    device: u8,
}

impl IterDevices {
    fn new(bus: u8, device: u8) -> Self {
        Self { bus, device }
    }
}

impl Iterator for IterDevices {
    type Item = Space;

    fn next(&mut self) -> Option<Self::Item> {
        for bus in self.bus as usize..Bus::MAX {
            for device in self.device..Device::MAX {
                if let Some(registers) = Registers::fetch(Bus::new(bus as u8), Device::new(device))
                {
                    self.bus = bus as u8;
                    self.device = device as u8 + 1;

                    return Some(Space::new(registers));
                }
            }
        }

        None
    }
}
