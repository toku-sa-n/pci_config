//! Structures related to capability registers.
pub mod common;
pub mod spec;

use {
    crate::space::{accessor::RegisterIndex, registers::Registers},
    common::{Common, Error, Type},
    spec::TypeSpec,
};

/// A struct representing a capability register.
pub struct Register<'a> {
    common: Common,
    spec: Option<TypeSpec<'a>>,
}
impl<'a> Register<'a> {
    /// Returns the type of this capability register.
    pub fn ty(&self) -> Result<Type, Error> {
        self.common.ty()
    }

    pub(crate) fn new(registers: &'a Registers, base: RegisterIndex) -> Self {
        let common = Common::new(registers, base);
        let spec = TypeSpec::new(registers, base, &common);

        Self { common, spec }
    }

    pub(crate) fn next_index(&self) -> RegisterIndex {
        self.common.next_index()
    }
}
