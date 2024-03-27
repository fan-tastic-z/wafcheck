pub mod safeline;


pub trait Plugin {
    fn check(&self, content: &str) -> bool;
    fn name(&self) -> String;
}


