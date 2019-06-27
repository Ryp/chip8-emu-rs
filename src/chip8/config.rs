pub struct Color
{
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

pub struct Palette
{
    pub primary: Color,
    pub secondary: Color,
}

pub struct EmuConfig
{
    pub debugMode: bool,
    pub palette: Palette,
    pub screenScale: u32,
}
