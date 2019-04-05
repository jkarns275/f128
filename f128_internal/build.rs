extern crate cc;

fn main() {
    cc::Build::new()
        .flag("-Bstatic")
        .flag("-lgfortran")
        .flag("-lquadmath")
        .file("src/f128.c")
        .compile("libf128.a");

    println!(r"cargo:rustc-flags=-l quadmath");
}
