#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

define_conversions! {
    Size, u32, u32, [width, height];
}