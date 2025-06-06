pub mod add;
pub mod show;
pub mod remove;
pub mod complete;

pub trait Command {
    fn execute(&self, args: &[String]) -> Result<(), Box<dyn std::error::Error>>;
}

