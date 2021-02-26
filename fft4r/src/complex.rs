//! FFT on complex units

use std::ptr::null_mut;

use kissfft_sys::{Complex32, bindings::{kiss_fft, kiss_fft_alloc, kiss_fft_cfg, kiss_fft_cpx}};
use libc::c_void;

use crate::Direction;

pub struct FftPlanner {
    cfg: kiss_fft_cfg,
    nfft: usize,
}
impl FftPlanner {
    /// Create a new planner.
    pub fn new(nfft: usize, direction: Direction) -> Self {
        assert_eq!(nfft & 1, 0, "{} is not an even number", nfft);
        let cfg =
            unsafe { kiss_fft_alloc(nfft as i32, direction as i32, null_mut(), null_mut()) };
        Self { cfg, nfft }
    }
    /// Perform the transformation. A new output buffer will be created on every call.
    pub fn transform(&mut self, input: &[Complex32]) -> Vec<Complex32> {
        assert_eq!(input.len(), self.nfft);
        let out_len = input.len();
        let mut output = vec![Complex32::default(); out_len];
        
        self.transform_with_buffer(input, &mut output);
        output.into_iter().map(Complex32::from).collect()
    }
    /// Perform the transformation from input to output.
    /// Output must be the same size in input.
    /// # Panics
    ///  - If `input` and `output` don't match in length
    pub fn transform_with_buffer(&mut self, input: &[Complex32], output: &mut [Complex32]) {
        assert_eq!(input.len(), output.len());
        let output: &mut [kiss_fft_cpx] = unsafe {
            std::mem::transmute(output)
        };
        let input: &[kiss_fft_cpx] = unsafe {
            std::mem::transmute(input)
        };
        unsafe {
            kiss_fft(self.cfg, input.as_ptr(), output.as_mut_ptr());
        }
    }
}

impl Drop for FftPlanner {
    fn drop(&mut self) {
        unsafe {
            libc::free(self.cfg as *mut c_void);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic() {
        let src = [Complex32::new(0.0, 0.0), Complex32::new(1.0, 0.0), Complex32::new(2.0,0.0), Complex32::new(3.0,0.0)];
        let expected = [
            Complex32::new(6.0, 0.0),
            Complex32::new(-2.0, 2.0),
            Complex32::new(-2.0, 0.0),
            Complex32::new(-2.0, -2.0),
        ];
        let mut fft = FftPlanner::new(4, Direction::default());
        let output = fft.transform(&src);
        assert_eq!(expected.as_ref(), output.as_slice());
    }
}