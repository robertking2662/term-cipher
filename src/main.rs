use std::env;

fn simple_hash(input: &str) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in input.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let text = if args.len() > 1 {
        args[1..].join(" ")
    } else {
        String::from("Hello, Open Source Rust!")
    };

    println!("==================================================");
    println!("  Rusty Hash & Checksum Engine (Rust)");
    println!("==================================================");
    println!("Input Text  : {}", text);
    println!("FNV-1a Hash : {:016x}", simple_hash(&text));
    println!("Byte Length : {} bytes", text.len());
    println!("==================================================");
}
