use super::accessor::{Accessor, Bus, Device, Function, RegisterIndex};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub(crate) struct Registers {
    bus: Bus,
    device: Device,
}

impl Registers {
    pub(crate) fn fetch(bus: Bus, device: Device) -> Option<Self> {
        let registers = Self { bus, device };
        if !registers.exist() {
            None
        } else {
            Some(registers)
        }
    }

    pub(crate) fn get(&self, index: RegisterIndex) -> u32 {
        let accessor = Accessor::new(self.bus, self.device, Function::zero(), index);
        accessor.read()
    }

    pub(crate) fn set(&self, index: RegisterIndex, value: u32) {
        let accessor = Accessor::new(self.bus, self.device, Function::zero(), index);
        accessor.write(value)
    }

    fn exist(&self) -> bool {
        self.get(RegisterIndex::zero()) != !0
    }
}
