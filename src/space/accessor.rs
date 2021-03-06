use {
    core::ops::Add,
    x86_64::instructions::port::{Port, PortWriteOnly},
};

pub(crate) struct Accessor {
    bus: Bus,
    device: Device,
    function: Function,
    index: RegisterIndex,
}

impl Accessor {
    const PORT_CONFIG_ADDR: PortWriteOnly<u32> = PortWriteOnly::new(0xcf8);
    const PORT_CONFIG_DATA: Port<u32> = Port::new(0xcfc);
    pub(crate) fn new(bus: Bus, device: Device, function: Function, index: RegisterIndex) -> Self {
        Self {
            bus,
            device,
            function,
            index,
        }
    }

    pub(crate) fn read(&self) -> u32 {
        let mut addr = Self::PORT_CONFIG_ADDR;
        unsafe { addr.write(self.address()) }

        let mut data = Self::PORT_CONFIG_DATA;
        unsafe { data.read() }
    }

    pub(crate) fn write(&self, value: u32) {
        let mut addr = Self::PORT_CONFIG_ADDR;
        unsafe { addr.write(self.address()) }

        let mut data = Self::PORT_CONFIG_DATA;
        unsafe { data.write(value) }
    }

    fn address(&self) -> u32 {
        let bus = self.bus.as_u8() as u32;
        let device = self.device.as_u8() as u32;
        let function = self.device.as_u8() as u32;
        let index = self.index.as_u8() as u32;

        const VALID: u32 = 0x8000_0000;

        VALID | bus << 16 | device << 11 | function << 8 | index << 2
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub(crate) struct Bus(u8);
impl Bus {
    pub(crate) const MAX: usize = 256;
    pub(crate) fn new(bus: u8) -> Self {
        Self(bus)
    }

    pub(crate) fn as_u8(self) -> u8 {
        self.0
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub(crate) struct Device(u8);
impl Device {
    pub(crate) const MAX: u8 = 32;
    pub(crate) fn new(device: u8) -> Self {
        assert!(device < Self::MAX);
        Self(device)
    }

    fn as_u8(self) -> u8 {
        self.0
    }
}

#[derive(Copy, Clone)]
pub(crate) struct Function(u8);
impl Function {
    pub(crate) fn new(function: u8) -> Self {
        assert!(function < 8);
        Self(function)
    }

    pub(crate) fn zero() -> Self {
        Self::new(0)
    }

    fn as_u8(self) -> u8 {
        self.0
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub(crate) struct RegisterIndex(u8);
impl RegisterIndex {
    pub(crate) const MAX: usize = 64;

    pub(crate) fn new(index: u8) -> Self {
        assert!(index < Self::MAX as u8);
        Self(index)
    }

    pub(crate) fn zero() -> Self {
        Self::new(0)
    }

    pub(crate) fn is_zero(self) -> bool {
        self.0 == 0
    }

    pub(crate) fn as_u8(self) -> u8 {
        self.0
    }
}
impl Add<u8> for RegisterIndex {
    type Output = Self;
    fn add(self, rhs: u8) -> Self::Output {
        Self::new(self.0 + rhs)
    }
}
