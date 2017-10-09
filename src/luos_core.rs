use module::Module;

pub struct Core {}

impl Core {
    pub fn new() -> Core {
        Core {}
    }
    pub fn register(&self, _module: &Module) {}

    pub fn update(&self) {}
}
