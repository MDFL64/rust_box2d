fn main() {
    println!("cargo:rustc-link-lib=static=Box2D");
    if let Some(path) = std::env::var("BOX2D_LIB_DIR").ok() {
        println!("cargo:rustc-link-search=native={}", path);
    } else {
        let box2d_install_prefix = cmake::Config::new("Box2D")
            .define("CMAKE_MSVC_RUNTIME_LIBRARY","MultiThreaded")
            .define("BOX2D_SAMPLES", "OFF")
            .define("BOX2D_VALIDATE", "OFF")
            .define("BOX2D_UNIT_TESTS", "OFF")
            .build();
        println!(
            "cargo:rustc-link-search=native={}/lib",
            box2d_install_prefix.display()
        );
    };
}
