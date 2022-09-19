use std::error::Error;

pub mod windows;

pub trait Mixer {
    fn set_volume(&self, pid: u64, level: f64) -> Result<(), Box<dyn Error>>;
}
