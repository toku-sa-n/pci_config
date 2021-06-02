define_field!(HeaderType, u8, 3, 16, 0xff);
impl HeaderType {
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
