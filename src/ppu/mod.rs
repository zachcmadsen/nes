mod address;

// - PPU generates 240 lines of pixels (each 256 pixels long) (256 x 240)
// - A tile is an 8 x 8 region. A block is a 16 x 16 region comprised of four tiles
// each screen has 240 blocks and 960 tiles
// - CHR represents raw pixel art, without color or position and is defined in
// terms of tiles
// - Since the number of tiles that can fit in a memory page (256) is far less
// than the tiles on screen (960) tiles are repeated
// - A nametable assigns a CHR tile to a position on screen. Each position is a
// byte so the nametable takes up 960 bytes
// - A palette is 3 unique colors plus a shared background color. An image has
// a maximum of four palettes. Each block can have one palette( i.e., we have
// to separate each 16 x 16 region by color palette)
// - Attributes choose palette is used for each block. Attributes are 2 bits
// for each block and specify which of the four palettes to use. The attributes
// for an image take up 64 bytes (60 bytes + 4 wasted bytes)
// - The four main components of NES graphics: CHR, nametable, palette, and
// attributes

// - There are two nametables. They share the same CHR, but each have their own
// attributes. The two are either stacked on top or side-by-side (I think this
// is mirroring?).
// - The PPU supports pixel-at-a-time scrolling in both x and y directions in
// order to exploit two nametables. Scrolling is controlled by writing to a
// PPU register. Think of this as scrolling across nametables
// - Sprites can be positioned arbitrarily (not aligned like nametables)
// - Sprites have their own CHR page and set of 4 palettes. They have a
// 256-byte page of memory that lists each sprite's position and appearance.
// Each entry takes four bytes so there's a hard limit of 256 / 4 = 64 sprites
// on screen at a time
// - A sprite must be 8 x 8 since it's a tile in CHR (Actually, the PPU can
// enable 8 x 16 sprites for tall sprites)
// - For any given horizontal line of the screen, if more than 8 sprites
// appear, those that appear later in memory simply won't be rendered. To get
// around this, games will rotate the addresses of sprites in memory so that
// each sprite is rendered at least part of the time. This is why some games
// flicker when there are a lot of sprites

// The PPU does scanline based rendering, left to right, top to bottom. Once
// bottom corner is reached, a period called "vertical blank" or vblank happens. The
// PPU does this rendering automatically every frame. Most of the changes to
// nametables and palettes happen during vblank. Some changes to PPU
// memory/state happen during rendering though. You can change the scroll
// midscreen so that only the bottom part of the screen scrolls for example.

// The sprite at memory position zero is treated specially. If the sprite is
// rendered and one of its pixels overlaps a visible part of the background (I think this means it's onscreen?),
// the sprite0 flag is set (the so-called sprite 0 hit). Game code will position
// the sprite where it wants and then poll the flag. That way it knows exactly
// which scanline is being rendered.

// Bank switching can be done with CHR date, instantly replacing the tiles that
// nametables or sprites refer to. You could do this in the middle of a render
// to say render a HUD with different CHR than the level.

// A mapper needs to be able to intercept PPU writes? I guess the PPU has to
// go through the mapper too? It does. I guess MMC2 needs this

use bitflags::bitflags;

use self::address::Address;

// bitflags! {
//     pub struct Control: u8 {
//         // const X = 1;
//         // const Y = 1 << 1;
//         // const NN = Self::X.bits() | Self::Y.bits();

//         const MSB_X_SCROLL = 0b0000_0001;
//         const MSB_Y_SCROLL = 0b0000_0010;
//         const BASE_NAMETABLE_ADDRESS = Self::MSB_X_SCROLL.bits() | Self::MSB_Y_SCROLL.bits();
//         const VRAM_ADDRESS_INCREMENT = 0b0000_0100;
//         const SPRITE_PATTERN_TABLE_ADDRESS = 0b0000_1000;
//         const BACKGROUND_PATTERN_TABLE_ADDRESS = 0b0001_0000;
//         const SPRITE_SIZE = 0b0010_0000;
//         const MASTER_SLAVE_SELECT = 0b0100_0000;
//         const GENERATE_NMI = 0b1000_0000;

//     }
// }

const CONTROL_REGISTER_ADDRESS: u16 = 0x2000;
const MASK_REGISTER_ADDRESS: u16 = 0x2001;
const STATUS_REGISTER_ADDRESS: u16 = 0x2002;
const OAM_ADDRESS_REGISTER_ADDRESS: u16 = 0x2003;
const OAM_DATA_REGISTER_ADDRESS: u16 = 0x2004;
const SCROLL_REGISTER_ADDRESS: u16 = 0x2005;
const ADDRESS_REGISTER_ADDRESS: u16 = 0x2006;
const DATA_REGISTER_ADDRESS: u16 = 0x2007;

bitflags! {
    pub struct Status: u8 {
        const SPRITE_OVERFLOW = 0b0010_0000;
        const SPRITE_0_HIT = 0b0100_0000;
        const VBLANK = 0b1000_0000;
    }
}

bitflags! {
    pub struct Mask: u8 {
        const GREYSCALE = 0b0000_0001;
        const SHOW_BACKGROUND_LEFTMOST = 0b0000_0010;
        const SHOW_SPRITES_LEFTMOST = 0b0000_0100;
        const SHOW_BACKGROUND = 0b0000_1000;
        const SHOW_SPRITES = 0b0001_0000;
        const EMPHASIZE_RED = 0b0010_0000;
        const EMPHASIZE_GREEN = 0b0100_0000;
        const EMPHASIZE_BLUE = 0b1000_0000;
    }
}

// bitflags! {
//     pub struct Address: u16 {
//         const COARSE_X_SCROLL = 0b0000_0000_0001_1111;
//         const COARSE_Y_SCROLL = 0b0000_0011_1110_0000;
//         const NAMETABLE_SELECT = 0b0000_1100_0000_0000;
//         const FINE_Y_SCROLL = 0b0011_0000_0000_0000;
//     }
// }

#[derive(Clone, Copy)]
struct Control(u8);

impl Control {
    fn from_bits(bits: u8) -> Control {
        Control(bits)
    }

    fn base_nametable_address(&self) -> u8 {
        self.0 & 0b0000_0011
    }

    fn vram_address_increment(&self) -> u8 {
        self.0 & 0b0000_0100
    }

    fn get_vram_address_increment(&self) -> u16 {
        if self.vram_address_increment() == 0 {
            1
        } else {
            32
        }
    }

    fn generate_nmi(&self) -> u8 {
        self.0 & 0b1000_0000
    }

    fn will_generate_nmi(&self) -> bool {
        self.generate_nmi() != 0
    }
}

pub struct Ppu {
    control: Control,
    mask: Mask,
    status: Status,
    oam_address: u8,

    temp_vram_address: Address,
    vram_address: Address,
    fine_x_scroll: u8,

    latch: bool,
    io_bus: u8,

    in_vblank: bool,
    should_generate_nmi: bool,

    vram: [u8; 2048],
    palette: [u8; 32],
    oam: [u8; 256],
}

impl Ppu {
    pub fn read(&mut self, address: u16) -> u8 {
        match address {
            STATUS_REGISTER_ADDRESS => {
                self.io_bus = self.status.bits() | (self.io_bus & 0x1f);

                self.status.remove(Status::VBLANK);
                self.latch = false;
            }
            OAM_DATA_REGISTER_ADDRESS => {
                todo!()
            }
            DATA_REGISTER_ADDRESS => {
                todo!()
            }
            _ => (),
        }

        // Reading any PPU register returns the I/O bus's value.
        self.io_bus
    }

    pub fn write(&mut self, address: u16, data: u8) {
        // Writing to any PPU register loads a value into the I/O bus.
        self.io_bus = data;

        match address {
            CONTROL_REGISTER_ADDRESS => {
                // TODO: Writes to the control register are ignored for about
                // TODO: 30,000 cycles after power-up/reset.
                let old_control = self.control;
                self.control = Control::from_bits(data);

                if self.in_vblank
                    && self.status.contains(Status::VBLANK)
                    && !old_control.will_generate_nmi()
                    && self.control.will_generate_nmi()
                {
                    self.should_generate_nmi = true;
                }

                self.temp_vram_address.set_base_nametable_address(
                    self.control.base_nametable_address(),
                );
            }
            MASK_REGISTER_ADDRESS => {
                self.mask = Mask::from_bits_truncate(data);
            }
            OAM_ADDRESS_REGISTER_ADDRESS => {
                self.oam_address = data;
            }
            OAM_DATA_REGISTER_ADDRESS => {
                todo!()
            }
            SCROLL_REGISTER_ADDRESS => {
                if !self.latch {
                    self.temp_vram_address.set_coarse_x_scroll(data);
                    self.fine_x_scroll = data & 0b0000_0111;
                } else {
                    self.temp_vram_address.set_coarse_y_scroll(data);
                    self.temp_vram_address.set_fine_y_scroll(data);
                }

                self.latch = !self.latch;
            }
            ADDRESS_REGISTER_ADDRESS => {
                if !self.latch {
                    self.temp_vram_address.set_high_byte(data);
                } else {
                    self.temp_vram_address.set_low_byte(data);
                    self.vram_address = self.temp_vram_address;
                }

                self.latch = !self.latch;
            }
            DATA_REGISTER_ADDRESS => {
                // TODO: Handle reads/writes during rendering. Only a couple
                // TODO: games are known to read during rendering.
                self.vram[self.vram_address.bits() as usize] = data;
                // TODO: This is gross.
                self.vram_address =
                    Address::from_bits(self.vram_address.bits().wrapping_add(
                        self.control.get_vram_address_increment(),
                    ))
            }
            _ => (),
        }
    }
}
