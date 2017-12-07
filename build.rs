extern crate cc;

fn main() {
    //gcc::Config::new().file("src/f80.c").flag("-lmath").compile("libf80.a");("libf80.a", &["src/f80.c"]);
    cc::Build::new()
        .file("src/f128.c")
        .flag("-Bstatic")
        .flag("-lquadmath")
        .compile("libf128.a");
    //("libf128.a", &["src/f128.c"]);

    println!("cargo:rustc-flags=-l quadmath");
}
