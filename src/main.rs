fn main() {
    const MANIFEST: &'static str = env!("RUSTC --version");
    println!("Rust version {}", MANIFEST);
}
