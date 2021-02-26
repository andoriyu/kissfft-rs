#!/bin/sh

 bindgen wrapper.h -o src/bindings.rs --size_t-is-usize --opaque-type "kiss_fft_state" --whitelist-type "^kiss_.*" --whitelist-function "^kiss_.*"
 
 # --whitelist-function kiss_fftr_alloc --whitelist-function kiss_fftr