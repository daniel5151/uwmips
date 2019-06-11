fn main() {
    let project_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}/lib/", project_dir);
    println!("cargo:rustc-link-lib=static=mipsasm");
    println!("cargo:rustc-link-lib=dylib=stdc++");
}
