
pub trait CustomTrait {
    const NAME : &'static str;
    fn is_valid(input :&str) -> bool;
}