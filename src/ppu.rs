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
