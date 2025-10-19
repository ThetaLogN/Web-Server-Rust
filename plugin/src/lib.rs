// src/lib.rs

#[no_mangle]
pub extern "C" fn run() {
    println!("🚀 Plugin example eseguito con successo!");
    let sum = add(5, 7);
    println!("📦 Calcolo interno del plugin: 5 + 7 = {}", sum);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}