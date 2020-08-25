use crate::math::Float;

pub struct Color {
    pub r: Float,
    pub g: Float,
    pub b: Float,
}

impl Color {
    pub fn rgb(r: Float, g: Float, b: Float) -> Color {
        Color { r, g, b }
    }

    pub fn clamp(&self) -> Color {
        Color {
            r: self.r.max(0.).min(1.),
            g: self.g.max(0.).min(1.),
            b: self.b.max(0.).min(1.),
        }
    }

    pub fn as_u8(&self) -> (u8, u8, u8) {
        let c = self.clamp();
        let r = (c.r * u8::MAX as Float).round() as u8;
        let g = (c.g * u8::MAX as Float).round() as u8;
        let b = (c.b * u8::MAX as Float).round() as u8;
        (r, g, b)
    }

    pub fn as_vec_u8(&self) -> Vec<u8> {
        let (r, g, b) = self.as_u8();
        vec![r, g, b]
    }
}
