use bit_field::BitField;

define_field!(HeaderType, u8, 3, 16, 0xff);
impl HeaderType {
    /// Returns [`true`] if the device has multiple functions, and [`false`] otherwise.
    pub fn multiple_functions_supported(&self) -> bool {
        self.0.get_bit(7)
    }

    pub(crate) fn ty(self) -> Header {
        match self.0 & 0x7f {
            0 => Header::Standard,
            1 => Header::PciToPciBridge,
            2 => Header::CardBusBridge,
            _ => unreachable!(),
        }
    }
}

pub(crate) enum Header {
    Standard,
    PciToPciBridge,
    CardBusBridge,
}
