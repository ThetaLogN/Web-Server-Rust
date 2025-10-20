fn main() {
    println!("🚀 Plugin eseguito con successo!");
    let sum = add(5, 7);
    println!("📦 Calcolo interno: 5 + 7 = {}", sum);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}