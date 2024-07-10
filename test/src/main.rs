
fn increment_name() -> String {
    static mut COUNT : u32 = 0;
    let mut name = String::from("p");
    unsafe {
        name.push_str(&COUNT.to_string());
        COUNT += 1;
    }
    name
}

fn main() {
    {
        for _ in 0..(26 * 3) {
            println!("{}",  increment_name());
        }
    }
}
