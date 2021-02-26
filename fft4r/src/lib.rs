///! Rust wrapper around [KissFFT](https://github.com/mborgerding/kissfft). 
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Represents a FFT direction.
pub enum Direction {
    Forward = 0,
    Inverse = 1,
}

impl std::default::Default for Direction {
    fn default() -> Direction {
        Direction::Forward
    }
}

pub use kissfft_sys::Complex32;
pub mod real;
pub mod complex;