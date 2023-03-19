fn main() {
    println!("cargo:rustc-link-arg=-Wl,--export=init");
    println!("cargo:rustc-link-arg=-Wl,--export=apply");
    println!("cargo:rustc-link-arg=-Wl,--export=register");
    println!("cargo:rustc-link-arg=-Wl,--export=unregister");
    println!("cargo:rustc-link-arg=-Wl,--export=allocate");
    println!("cargo:rustc-link-arg=-Wl,--export=deallocate");
    // https://github.com/vmware-labs/webassembly-language-runtimes/issues/79
    println!("cargo:rustc-link-arg=-Wl,-z,stack-size=524288");
    println!("cargo:rustc-link-arg=-mexec-model=reactor");
}
