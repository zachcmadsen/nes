use crate::mapper::{Map, Mapper0};

const HEADER_SIZE: usize = 16;
const PRG_ROM_BANK_SIZE: usize = 0x4000;
const CHR_ROM_BANK_SIZE: usize = 0x2000;
const PRG_RAM_BANK_SIZE: usize = 0x2000;
const HEADER_PREAMBLE: [u8; 4] = [0x4e, 0x45, 0x53, 0x1a];

pub struct Cartridge {
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
    prg_ram: Vec<u8>,
    mapper: Box<dyn Map>,
}

impl Cartridge {
    pub fn new(data: &[u8]) -> Cartridge {
        if data.len() < HEADER_SIZE {
            panic!()
        }

        if data[0..=3] != HEADER_PREAMBLE {
            panic!("invalid header preamble")
        }

        let num_prg_rom_banks = data[4];
        let num_chr_rom_banks = data[5];
        let num_prg_ram_banks = data[8];

        let prg_rom_len = num_prg_rom_banks as usize * PRG_ROM_BANK_SIZE;
        let chr_rom_len = num_chr_rom_banks as usize * CHR_ROM_BANK_SIZE;
        let prg_ram_len = num_prg_ram_banks as usize * PRG_RAM_BANK_SIZE;

        if data.len() < HEADER_SIZE + prg_rom_len + chr_rom_len + prg_ram_len {
            panic!()
        }

        // TODO: Handle nametable mirroring and the presence of a trainer.
        let mapper_number = data[7] & 0xf0 | data[6] >> 4;
        let mapper = match mapper_number {
            0 => Mapper0 {
                has_one_bank: num_prg_rom_banks == 1,
            },
            _ => unimplemented!(),
        };

        Cartridge {
            prg_rom: data[HEADER_SIZE..(HEADER_SIZE + prg_rom_len)].to_vec(),
            chr_rom: data[(HEADER_SIZE + prg_rom_len)
                ..(HEADER_SIZE + prg_rom_len + chr_rom_len)]
                .to_vec(),
            prg_ram: vec![0; prg_ram_len],
            mapper: Box::new(mapper),
        }
    }

    pub fn read_byte(&mut self, address: u16) -> u8 {
        let mapped_address = self.mapper.map(address);
        self.prg_rom[mapped_address as usize]
    }

    pub fn write_byte(&mut self, address: u16, data: u8) {
        let mapped_address = self.mapper.map(address);
        match mapped_address {
            0x6000..=0x7fff => {
                self.prg_ram[mapped_address as usize] = data;
            }
            _ => panic!("can't write to ROM"),
        }
    }
}
