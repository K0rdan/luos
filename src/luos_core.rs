use module::Module;


/// # Luos Core
/// It can host multiple modules sensors/effectors.
/// You have to register every local modules to the core.
/// Abstracts all communication with the other cores so each module can communicate with the the other modules whether they are local or not. To do that, you simply have to call the update method.
pub struct Core {}

impl Core {
    pub fn new() -> Core {
        Core {}
    }
    pub fn register(&self, _module: &Module) {
        _module.alias();
    }
}
