use super::Map;

pub(crate) struct Mapper0 {
    pub(crate) has_one_bank: bool,
}

impl Map for Mapper0 {
    fn map(&mut self, address: u16) -> u16 {
        if self.has_one_bank {
            (address & 0xbfff) - 0x8000
        } else {
            address - 0x8000
        }
    }
}
