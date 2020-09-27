// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Structures related to capability registers.
pub mod common;
pub mod spec;

use {
    crate::space::{accessor::RegisterIndex, registers::Registers},
    common::Common,
    spec::TypeSpec,
};

/// A struct representing a capability register.
pub struct Register<'a> {
    common: Common,
    spec: Option<TypeSpec<'a>>,
}
impl<'a> Register<'a> {
    pub(crate) fn new(registers: &'a Registers, base: RegisterIndex) -> Self {
        let common = Common::parse_registers(registers, base);
        let spec = TypeSpec::new(registers, base, &common);

        Self { common, spec }
    }

    pub(crate) fn next_index(&self) -> RegisterIndex {
        self.common.next_index()
    }
}
