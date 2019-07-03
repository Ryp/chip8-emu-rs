#[derive(Default)]
pub struct Color
{
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[derive(Default)]
pub struct Palette
{
    pub primary: Color,
    pub secondary: Color,
}

#[derive(Default)]
pub struct EmuConfig
{
    pub debug_mode: bool,
    pub palette: Palette,
    pub screen_scale: u32,
}
