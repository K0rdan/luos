use luos_core::Core;

mod button;
pub use module::button::Button;

mod led;
pub use module::led::Led;

pub trait Module {
    fn update(&mut self, core: &Core);
}
