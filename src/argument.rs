#[derive(Debug)]
pub struct ArgumentMeta {
    pub name: String,
    pub prefix: String,
    pub help: String,
}

pub trait Argument {
    fn meta(&self) -> &ArgumentMeta;
}