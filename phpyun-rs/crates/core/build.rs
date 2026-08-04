fn main() {
    // rust-i18n embeds the shared locale directory through a procedural macro,
    // so Cargo does not otherwise know that a locale edit requires rebuilding
    // the core crate. Keep cargo-watch and normal incremental builds correct.
    println!("cargo:rerun-if-changed=../../locales");
}
