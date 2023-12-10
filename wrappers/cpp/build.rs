fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/bbs.cc")
        .flag_if_supported("-std=c++14")
        .compile("paring_crypto_cpp");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/bbs.cc");
    println!("cargo:rerun-if-changed=include/bbs.h");
}
