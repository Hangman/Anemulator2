#[derive(Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn r_as_u8(&self) -> u8 {
        (self.r * 255f32) as u8
    }

    pub fn g_as_u8(&self) -> u8 {
        (self.g * 255f32) as u8
    }

    pub fn b_as_u8(&self) -> u8 {
        (self.b * 255f32) as u8
    }

    pub fn a_as_u8(&self) -> u8 {
        (self.a * 255f32) as u8
    }
}
