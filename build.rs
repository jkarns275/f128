extern crate cc;

fn main() {
    //gcc::Config::new().file("src/f80.c").flag("-lmath").compile("libf80.a");("libf80.a", &["src/f80.c"]);
    cc::Build::new()
        .flag("-Bstatic")
        .flag("-lgfortran")
        .flag("-lquadmath")
        .file("src/f128.c")
        .compile("libf128.a");
    //("libf128.a", &["src/f128.c"]);

    println!(r"cargo:rustc-flags=-l quadmath");
    //println!(r"cargo:rustc-link-search=C:\Program Files\msys64\usr\lib\gcc\x86_64-pc-msys\6.3.0")
}
