#[repr(C)]
#[derive(Clone, Debug)]
pub struct Filter {
    pub category_bits: u16,
    pub mask_bits: u16,
    pub group_index: i16,
}

#[repr(transparent)]
pub struct HexColor(u32);

impl HexColor {
    pub fn to_floats(&self) -> [f32;3] {
        let r = ((self.0 >> 16) & 0xFF) as f32 / 255.0;
        let g = ((self.0 >> 8) & 0xFF) as f32 / 255.0;
        let b = (self.0 & 0xFF) as f32 / 255.0;

        [r,g,b]
    }
}
