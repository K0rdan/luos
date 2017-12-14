//! Luos Core - handles all communication within drivers and with other cores.

use driver::Driver;

/// Luos Core - handles all communication within drivers and with other cores.
///
/// It can host multiple drivers sensors/effectors.
///
/// You have to register every local drivers to the core.
///
/// Abstracts all communication with the other cores so each driver can communicate with the the other drivers whether they are local or not.
pub struct Core {}

impl Core {
    pub fn new() -> Core {
        Core {}
    }
    pub fn register(&self, _: &Driver) {}
}
