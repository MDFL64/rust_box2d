#[repr(C)]
#[derive(Clone, Debug)]
pub struct Filter {
    pub category_bits: u16,
    pub mask_bits: u16,
    pub group_index: i16,
}
