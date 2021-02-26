//! Real-optimized FFTs (returns the positive half-spectrum: (nfft/2+1) complex frequency bins)
//! The real (i.e. not complex) optimization code only works for even length ffts. It does two half-length
//! FFTs in parallel (packed into real&imag), and then combines them via twiddling. The result is nfft/2+1
//! complex frequency bins from DC to Nyquist. If you don't know what this means, search the web because neither do I.
//!
//! [Read more](https://stackoverflow.com/questions/52387673/what-is-the-difference-between-numpy-fft-fft-and-numpy-fft-rfft).
use libc::c_void;

use kissfft_sys::{
    bindings::{kiss_fftr, kiss_fftr_alloc, kiss_fftr_cfg, kiss_fft_cpx},
    Complex32,
};
use super::Direction;
use std::ptr::null_mut;
/// FFT algorithm instances.
/// # Example
/// ```
/// use fft4r::{Complex32, real::FftPlanner, Direction};
/// let src = [0f32, 1.0, 2.0, 3.0];
/// let expected = [
///   Complex32::new(6.0, 0.0),
///   Complex32::new(-2.0, 2.0),
///   Complex32::new(-2.0, 0.0)
/// ];
/// let mut fft = FftPlanner::new(4, Direction::default());
/// let output = fft.transform(&src);
/// assert_eq!(expected.as_ref(), output.as_slice());
/// ```
pub struct FftPlanner {
    cfg: kiss_fftr_cfg,
    nfft: usize,
}

impl FftPlanner {
    /// Create a new planner.
    /// # Panics
    ///  - If `nfft` is not even
    pub fn new(nfft: usize, direction: Direction) -> Self {
        assert_eq!(nfft & 1, 0, "{} is not an even number", nfft);
        let cfg =
            unsafe { kiss_fftr_alloc(nfft as i32, direction as i32, null_mut(), null_mut()) };
        Self { cfg, nfft }
    }
    /// Perform the transformation. A new output buffer will be created on every call.
    pub fn transform(&mut self, input: &[f32]) -> Vec<Complex32> {
        assert_eq!(input.len(), self.nfft);
        let out_len = (input.len() / 2) + 1;
        let mut output = vec![Complex32::default(); out_len];
        
        self.transform_with_buffer(input, &mut output);
        output.into_iter().map(Complex32::from).collect()
    }
    /// Perform the transformation.
    /// 
    /// NOTE: Output buffet is a list of `kiss_fft_cpx` and not `Complex32`
    pub fn transform_with_buffer(&mut self, input: &[f32], output: &mut [Complex32]) {
        let output:&mut [kiss_fft_cpx] = unsafe {
            std::mem::transmute(output)
        };
        unsafe {
            kiss_fftr(self.cfg, input.as_ptr(), output.as_mut_ptr());
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
mod tests {
    use super::*;
    #[test]
    fn basic_test() {
        let src = [0f32, 1.0, 2.0, 3.0];

        let expected = [
            Complex32::new(6.0, 0.0),
            Complex32::new(-2.0, 2.0),
            Complex32::new(-2.0, 0.0)
        ];
        let mut fft = FftPlanner::new(4, Direction::default());
        let output = fft.transform(&src);
        assert_eq!(expected.as_ref(), output.as_slice());
    }
}