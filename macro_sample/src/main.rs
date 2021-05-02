use macro_export::format;
use macro_support::CustomTrait;

#[format]
fn test(foo:&str) -> bool {
    true
}
// struct test_t;

//         impl macro_support::CustomTrait for test_t
//         {
//             const NAME : &'static str = "test";
//             fn is_valid(&self,foo:&str) -> bool {
//                 true
//             }
//         }

//         const test : test_t = test_t {};

//         impl ToString for test_t {
//             fn to_string(&self) -> String {
//                 format!("format: {}", <Self as macro_support::CustomTrait>::NAME).to_string()
//             }
//         }

fn main() {

    println!("Hello, world!{}", "foo");
}
