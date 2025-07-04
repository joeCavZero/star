use star::star::core::Star;

fn main() {
    println!("Running Star VM...");
    let mut star = Star::new();
    star.load(&"test.asm".to_string());
}
