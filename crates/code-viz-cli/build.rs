fn main() {
    // libgit2-sys 0.16.x omits advapi32 from its Windows link list,
    // causing unresolved symbols (GetNamedSecurityInfoW, RegCloseKey, etc.).
    // Explicitly link it here until git2 is upgraded to a version that
    // includes the fix.
    #[cfg(target_os = "windows")]
    println!("cargo:rustc-link-lib=advapi32");
}
