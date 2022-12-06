const COARSE_X_SCROLL_MASK: u16 = 0b0000_0000_0001_1111;
const COARSE_Y_SCROLL_MASK: u16 = 0b0000_0011_1110_0000;
const BASE_NAMETABLE_ADDRESS_MASK: u16 = 0b0000_1100_0000_0000;
const FINE_Y_SCROLL_MASK: u16 = 0b0011_0000_0000_0000;
const HIGH_BYTE_MASK: u16 = 0xff00;
const LOW_BYTE_MASK: u16 = 0x00ff;

#[derive(Clone, Copy)]
pub(crate) struct Address(u16);

impl Address {
    pub fn from_bits(bits: u16) -> Address {
        Address(bits)
    }

    pub fn bits(&self) -> u16 {
        self.0
    }

    pub fn coarse_x_scroll(&self) -> u8 {
        (self.0 & COARSE_X_SCROLL_MASK) as u8
    }

    pub fn set_coarse_x_scroll(&mut self, bits: u8) {
        self.0 = (self.0 & !COARSE_X_SCROLL_MASK)
            | (bits as u16 & COARSE_X_SCROLL_MASK);
    }

    pub fn coarse_y_scroll(&self) -> u8 {
        ((self.0 & COARSE_Y_SCROLL_MASK) >> 5) as u8
    }

    pub fn set_coarse_y_scroll(&mut self, bits: u8) {
        self.0 = (self.0 & !COARSE_Y_SCROLL_MASK)
            | (bits as u16 & COARSE_Y_SCROLL_MASK);
    }

    pub fn base_nametable_address(&self) -> u8 {
        ((self.0 & BASE_NAMETABLE_ADDRESS_MASK) >> 10) as u8
    }

    pub fn set_base_nametable_address(&mut self, bits: u8) {
        self.0 = (self.0 & !BASE_NAMETABLE_ADDRESS_MASK)
            | (bits as u16 & BASE_NAMETABLE_ADDRESS_MASK) << 10;
    }

    pub fn fine_y_scroll(&self) -> u8 {
        ((self.0 & FINE_Y_SCROLL_MASK) >> 12) as u8
    }

    pub fn set_fine_y_scroll(&mut self, bits: u8) {
        self.0 = (self.0 & !FINE_Y_SCROLL_MASK)
            | (bits as u16 & FINE_Y_SCROLL_MASK);
    }

    pub fn set_high_byte(&mut self, byte: u8) {
        self.0 = (self.0 & LOW_BYTE_MASK) | ((byte as u16) << 8) & 0x3fff;
    }

    pub fn set_low_byte(&mut self, byte: u8) {
        self.0 = (self.0 & HIGH_BYTE_MASK) | byte as u16;
    }
}
