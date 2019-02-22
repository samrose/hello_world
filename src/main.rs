fn main() {
    const MANIFEST: &'static str = env!("RUSTC");
    println!("Rust version {}", MANIFEST);
}
