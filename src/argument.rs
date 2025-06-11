#[derive(Debug)]
pub struct ArgumentMeta {
    pub name: String,
    pub prefix: String, // None für positionale Argumente
    pub help: String,
}

pub trait Argument {
    fn meta(&self) -> &ArgumentMeta;
}