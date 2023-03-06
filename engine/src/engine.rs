use env_logger;
pub use log::*;
pub mod graphics;
pub use gl::*;

pub fn init()
{
    env_logger::init();
}