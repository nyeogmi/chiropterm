use crate::aliases::PixelSize;

use super::{cell::SemanticContent, sprite::{Tile, TileSet}};

const BITMAP: &'static [u8; 0x1000] = include_bytes!("font.bin");
const BITMAP_SMALL: &'static [u8; 0x800] = include_bytes!("font_small.bin");
const BITMAP_FAT: &'static [u8; 0x2000] = include_bytes!("font_fat.bin");

const FONT: TileSet<'static> = TileSet {
    buf: BITMAP,
    overall_size: PixelSize::new(256, 128),
};

const FONT_SMALL: TileSet<'static> = TileSet {
    buf: BITMAP_SMALL,
    overall_size: PixelSize::new(128, 128),
};

const FONT_FAT: TileSet<'static> = TileSet {
    buf: BITMAP_FAT,
    overall_size: PixelSize::new(256, 256),
};

pub fn eval(content: SemanticContent) -> Tile {
    match content {
        SemanticContent::Blank => { Tile([0; 8]) }
        SemanticContent::TopHalf(u) => { FONT.tile((u as usize) * 2) }
        SemanticContent::BottomHalf(u) => { FONT.tile((u as usize) * 2 + 1) }

        SemanticContent::Small(u) => { FONT_SMALL.tile(u as usize) }
        
        SemanticContent::SetTL(u) => { FONT.tile((u as usize) * 2).left() }
        SemanticContent::SetTR(u) => { FONT.tile((u as usize) * 2).right() }
        SemanticContent::SetBL(u) => { FONT.tile((u as usize) * 2 + 1).left() }
        SemanticContent::SetBR(u) => { FONT.tile((u as usize) * 2 + 1).right() }

        SemanticContent::FatTL(u) => { FONT_FAT.tile((u as usize) * 4) }
        SemanticContent::FatTR(u) => { FONT_FAT.tile((u as usize) * 4 + 1) }
        SemanticContent::FatBL(u) => { FONT_FAT.tile((u as usize) * 4 + 2) }
        SemanticContent::FatBR(u) => { FONT_FAT.tile((u as usize) * 4 + 3) }
    }
}