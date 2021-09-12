use euclid::{Point2D, Rect, Size2D, rect};

use super::{cell::SemanticContent, sprite::{Sprite, TileSet}};

const BITMAP: &'static [u8; 0x1000] = include_bytes!("font.bin");

const FONT: TileSet<'static> = TileSet {
    buf: BITMAP,
    overall_size: Size2D::new(256, 128),
    tile_size: Size2D::new(8, 16),
};

pub fn eval(content: SemanticContent) -> Sprite<'static> {
    match content {
        SemanticContent::TopHalf(u) => {
            let big_tile = FONT.tile(u as usize);
            big_tile.subsprite(rect(0, 0, 8, 8))
        }
        SemanticContent::BottomHalf(u) => {
            let big_tile = FONT.tile(u as usize);
            big_tile.subsprite(rect(0, 8, 8, 8))
        }
    }
}