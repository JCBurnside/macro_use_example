use macro_export::format;

#[format]
fn test(_:&str) -> bool {
    true
}

fn main() {
    println!("Hello, world!");
}
