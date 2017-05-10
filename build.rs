extern crate gcc;

fn main() {
    //gcc::Config::new().file("src/f80.c").flag("-lmath").compile("libf80.a");("libf80.a", &["src/f80.c"]);
    gcc::Config::new().file("src/f128.c").flag("-lquadmath").compile("libf128.a");("libf128.a", &["src/f128.c"]);

    println!("cargo:rustc-flags=-l quadmath");
}
