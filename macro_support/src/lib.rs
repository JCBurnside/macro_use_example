
pub trait CustomTrait {
    const NAME : &'static str;
    fn is_valid(&self,input :&str) -> bool;
}