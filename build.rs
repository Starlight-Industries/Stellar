/// test
fn main() {
    let is_nightly = version_check::is_feature_flaggable() == Some(true);
    /// if
    if is_nightly {
        println!("cargo:warning=Building with nightly features enabled");
        println!("cargo:rustc-cfg=feature=\"nightly\"");
    } else {
        // println!("cargo:rustc-cfg=not(nightly)");
        println!("cargo:warning=Building with nightly features disabled");
    }
}
