use bog::{Bus, Pins};

use crate::NromCartridge;

const NES_RAM_SIZE: usize = 0x0800;

pub struct NesBus {
    ram: [u8; NES_RAM_SIZE],
    pub cartridge: NromCartridge,
}

impl NesBus {
    pub fn new(cartridge: NromCartridge) -> NesBus {
        NesBus {
            ram: [0; NES_RAM_SIZE],
            cartridge,
        }
    }

    fn read(&mut self, pins: &mut Pins) {
        // TODO: Rewrite as pins.data = match pins.address { ... } once the
        // other match arms are implemented.
        match pins.address {
            0x0000..=0x1fff => {
                pins.data = self.ram[(pins.address & 0x07ff) as usize]
            }
            0x2000..=0x3fff => (),
            0x4000..=0x401f => (),
            0x4020..=0xffff => {
                pins.data = self.cartridge.read_prg(pins.address)
            }
        }
    }

    fn write(&mut self, pins: &mut Pins) {
        match pins.address {
            0x0000..=0x1fff => {
                self.ram[(pins.address & 0x07ff) as usize] = pins.data
            }
            0x2000..=0x3fff => (),
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
            true => self.read(pins),
            false => self.write(pins),
        }
    }
}
