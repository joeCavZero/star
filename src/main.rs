use star::star::core::Star;

fn main() {
    println!("Running Star VM...");
    let mut star = Star::new();
    star.load_memory(&"test.asm".to_string());
}
