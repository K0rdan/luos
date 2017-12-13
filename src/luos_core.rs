//! Luos Core - handles all communication within modules and with other cores.

use driver::Driver;

/// Luos Core - handles all communication within modules and with other cores.
///
/// It can host multiple modules sensors/effectors.
///
/// You have to register every local modules to the core.
///
/// Abstracts all communication with the other cores so each module can communicate with the the other modules whether they are local or not.
pub struct Core {}

impl Core {
    pub fn new() -> Core {
        Core {}
    }
    pub fn register(&self, _: &Driver) {}
}
