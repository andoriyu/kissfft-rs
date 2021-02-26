pub use num_complex::Complex32;
pub mod bindings;

impl From<bindings::kiss_fft_cpx> for Complex32 {
    fn from(src: bindings::kiss_fft_cpx) -> Complex32 {
        Complex32::new(src.r, src.i)
    }
}

impl std::default::Default for bindings::kiss_fft_cpx {
    fn default() -> bindings::kiss_fft_cpx {
        bindings::kiss_fft_cpx {
            r: 0.0,
            i: 0.0,
        }
    }
}

impl bindings::kiss_fft_cpx {
    pub fn new(r: f32, i: f32) -> bindings::kiss_fft_cpx {
        bindings::kiss_fft_cpx {
            r, i
        }
    }
}