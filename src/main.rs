fn main() {
    const MANIFEST: &'static str = env!("CARGO");
    println!("Rust version {}", MANIFEST);
}
