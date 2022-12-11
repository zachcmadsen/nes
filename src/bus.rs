use bog::{Bus, Pins};

use crate::Cartridge;

const NES_RAM_SIZE: usize = 0x0800;

pub struct NesBus {
    ram: [u8; NES_RAM_SIZE],
    cartridge: Cartridge,
}

impl NesBus {
    pub fn new(cartridge: Cartridge) -> NesBus {
        NesBus {
            ram: [0; NES_RAM_SIZE],
            cartridge,
        }
    }

    fn read_byte(&mut self, pins: &mut Pins) {
        pins.data = match pins.address {
            0x0000..=0x1fff => self.ram[(pins.address & 0x07ff) as usize],
            0x2000..=0x3fff => unimplemented!("PPU registers"),
            0x4000..=0x401f => unimplemented!("APU and IO registers"),
            0x4020..=0xffff => self.cartridge.read_prg(pins.address),
        }
    }

    fn write_byte(&mut self, pins: &mut Pins) {
        match pins.address {
            0x0000..=0x1fff => {
                self.ram[(pins.address & 0x07ff) as usize] = pins.data
            }
            0x2000..=0x3fff => unimplemented!("PPU registers"),
            0x4000..=0x401f => (),
            0x4020..=0xffff => {
                self.cartridge.write_prg(pins.address, pins.data)
            }
        }
    }
}

impl Bus for NesBus {
    fn tick(&mut self, pins: &mut Pins) {
        match pins.rw {
            true => self.read_byte(pins),
            false => self.write_byte(pins),
        }
    }
}
