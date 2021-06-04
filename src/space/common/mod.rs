use bit_field::BitField;
use core::convert::TryInto;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

mod class;
pub(crate) mod header;
mod id;

use {
    crate::space::registers::Registers,
    class::Class,
    header::{Header, HeaderType},
    id::Id,
};

/// The fields of PCI headers all PCI devices have.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Common<'a> {
    registers: &'a Registers,
}
impl<'a> Common<'a> {
    pub(crate) fn new(registers: &'a Registers) -> Self {
        Self { registers }
    }

    pub(crate) fn header_type(&self) -> Header {
        let header_type = HeaderType::new(self.registers);
        header_type.ty()
    }

    pub(crate) fn capability_pointer_exists(&self) -> bool {
        self.status().capability_pointer_exists()
    }

    fn status(&self) -> Status {
        Status::new(self.registers)
    }
}

define_field!(Command, u16, 1, 0, 0xffff);
define_field!(Status, u16, 1, 16, 0xffff);
define_field!(Interface, u8, 2, 8, 0xff);
define_field!(CacheLineSize, u8, 3, 0, 0xff);
define_field!(LatencyTimer, u8, 3, 8, 0xff);
define_field!(Bist, u8, 3, 24, 0xff);

impl Status {
    /// Returns [`true`] if the device implements the Capability list, and [`false`] otherwise.
    pub fn capability_pointer_exists(self) -> bool {
        self.0.get_bit(4)
    }

    /// Returns the frequency that the device is capable of running at.
    pub fn capable_at(self) -> RunningMHz {
        if self.0.get_bit(5) {
            RunningMHz::At66MHz
        } else {
            RunningMHz::At33MHz
        }
    }

    /// Returns [`true`] if the device supports the fast back-to-back, and [`false`] otherwise.
    pub fn fast_back_to_back_supported(self) -> bool {
        self.0.get_bit(7)
    }

    /// Returns [`true`] if the following conditions are met, and [`false`] otherwise.
    ///
    /// - The bus agent asserted `PERR#` on a read, or observed an assertion of `PERR#` on a write.
    /// - The agent setting this bit acted as the master for the operation the error occured in.
    /// - The Parity Error Response bit of the Command Register is set.
    ///
    /// This bit is implemented only for bus masters.
    pub fn master_data_parity_error(self) -> bool {
        self.0.get_bit(8)
    }

    /// Returns the `DEVSEL` timing.
    pub fn devsel_timing(self) -> DevselTiming {
        let v = self.0.get_bits(9..=10);
        let v = FromPrimitive::from_u16(v);
        v.expect("The value represents Reserved.")
    }

    /// Returns [`true`] if the device detects a parity error, and [`false`] otherwise.
    pub fn parity_error_detected(self) -> bool {
        self.0.get_bit(15)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, FromPrimitive)]
pub enum RunningMHz {
    /// The device can run at 66 MHz.
    At66MHz,
    /// The device can run at 33 MHz.
    At33MHz,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, FromPrimitive)]
pub enum DevselTiming {
    Fast = 0b00,
    Medium = 0b01,
    Slow = 0b10,
}
