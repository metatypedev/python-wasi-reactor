fn main() {
    println!("cargo:rustc-link-arg=-Wl,--export=init");
    println!("cargo:rustc-link-arg=-Wl,--export=apply_lambda");
    println!("cargo:rustc-link-arg=-Wl,--export=register_lambda");
    println!("cargo:rustc-link-arg=-Wl,--export=unregister_lambda");
    println!("cargo:rustc-link-arg=-Wl,--export=apply_def");
    println!("cargo:rustc-link-arg=-Wl,--export=register_module");
    println!("cargo:rustc-link-arg=-Wl,--export=allocate");
    println!("cargo:rustc-link-arg=-Wl,--export=deallocate");
    // https://github.com/vmware-labs/webassembly-language-runtimes/issues/79
    println!("cargo:rustc-link-arg=-Wl,-z,stack-size=524288");
    println!("cargo:rustc-link-arg=-mexec-model=reactor");
}
