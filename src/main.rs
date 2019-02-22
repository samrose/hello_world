fn main() {
    const MANIFEST: &'static str = env!("CARGO_MANIFEST_DIR");
    println!("Manifest Dir {}", MANIFEST);
}
