use euclid::{Size2D};

use super::{cell::SemanticContent, sprite::{Tile, TileSet}};

const BITMAP: &'static [u8; 0x1000] = include_bytes!("font.bin");

const FONT: TileSet<'static> = TileSet {
    buf: BITMAP,
    overall_size: Size2D::new(256, 128),
};

pub fn eval(content: SemanticContent) -> Tile {
    match content {
        SemanticContent::TopHalf(u) => { FONT.tile((u as usize) * 2) }
        SemanticContent::BottomHalf(u) => { FONT.tile((u as usize) * 2 + 1) }
    }
}