extern crate cc;

fn main() {
    cc::Build::new()
        .file("./kissfft/kiss_fftr.c")
        .file("./kissfft/kiss_fft.c")
        //.define("FOO", Some("bar"))
        .opt_level(2)
        .include("./kissfft/")
        .static_flag(true)
        .compile("kissfft");
}