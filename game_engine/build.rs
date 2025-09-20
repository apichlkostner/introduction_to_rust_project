use cc;

fn main() {
    println!("cargo:rerun-if-changed=../opengl_wrapper_lib/opengl_wrapper_lib.c");

    cc::Build::new()
        .file("../opengl_wrapper_lib/opengl_wrapper_lib.c")
        .include("../opengl_wrapper_lib")
        .shared_flag(true)
        .compile("opengl_wrapper_lib");

    println!("cargo:rustc-link-lib=dylib=opengl_wrapper_lib");
    println!(
        "cargo:rustc-link-search=native={}",
        std::env::var("OUT_DIR").unwrap()
    );
    println!("cargo:rustc-link-lib=dylib=glfw");
    println!("cargo:rustc-link-lib=dylib=GL");
}
