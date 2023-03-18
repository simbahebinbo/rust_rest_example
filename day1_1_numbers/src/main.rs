fn main() {
    // Rust floats f64 precision traps
    let x = 0.1 + 0.2;
    let y: f32 = 0.1 + 0.2;
    println!("{}", 0.1 + 0.2); // 0.30000000000000004
    println!("{}", x); //0.30000000000000004
    println!("{}", y); //0.3
}
