#[derive(Debug, Clone, Copy)]
pub struct Example {
    pub name: &'static str,
    pub elf: &'static [u8],
}

pub fn examples() -> &'static [Example] {
    &[]
}

pub fn get_example(_name: &str) -> Option<&'static [u8]> {
    None
}
