fn main() {
    let out = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let boot_o = out.join("boot.o");
    let boot_a = out.join("libboot.a");

    let s = std::process::Command::new("llvm-mc")
        .args([
            "-triple=x86_64-unknown-none",
            "-filetype=obj",
            "-o",
            boot_o.to_str().unwrap(),
            "boot/boot.s",
        ])
        .status()
        .unwrap();
    assert!(s.success(), "llvm-mc failed");

    let s = std::process::Command::new("ar")
        .args(["crs", boot_a.to_str().unwrap(), boot_o.to_str().unwrap()])
        .status()
        .unwrap();
    assert!(s.success(), "ar failed");

    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rustc-link-lib=boot");
}
