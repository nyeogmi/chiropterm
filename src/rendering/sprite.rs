use euclid::{Point2D, Rect, Size2D};

use crate::spaces::PixelSpace;

pub struct TileSet<'a> {
    pub buf: &'a [u8],
    pub overall_size: Size2D<u16, PixelSpace>,
    pub tile_size: Size2D<u16, PixelSpace>,
}

#[derive(Clone, Copy)]
pub struct Sprite<'a> {
    buf: &'a [u8],
    overall_size: Size2D<u16, PixelSpace>,
    drawfrom_rect: Rect<u16, PixelSpace>,
    // TODO: Allow cropping
}

impl<'a> TileSet<'a> {
    pub fn tile(&self, ix: usize) -> Sprite<'a> {
        let n_tiles_x = (self.overall_size.width / self.tile_size.width) as usize;
        let n_tiles_y = (self.overall_size.height / self.tile_size.height) as usize;
        let n_tiles = n_tiles_x * n_tiles_y;

        if ix >= n_tiles {
            return Sprite {
                buf: self.buf,
                overall_size: self.overall_size,
                drawfrom_rect: Rect::new(Point2D::zero(), Size2D::zero()),
            }
        }

        let x = (ix % n_tiles_x) as u16;
        let y = (ix / n_tiles_x) as u16;

        let rect = Rect::new(
            Point2D::new(x * self.tile_size.width, y * self.tile_size.height),
            self.tile_size,
        );

        Sprite { 
            buf: self.buf,
            overall_size: self.overall_size,
            drawfrom_rect: rect, 
        }
    }
}

impl<'a> Sprite<'a> {
    pub fn subsprite(&self, sub: Rect<u16, PixelSpace>) -> Self {
        Sprite {
            buf: self.buf,
            overall_size: self.overall_size,
            drawfrom_rect: 
                self.drawfrom_rect.intersection(&sub.translate(self.drawfrom_rect.origin.to_vector()))
                    .unwrap_or(Rect::new(Point2D::zero(), Size2D::zero())),
        }
    }

    #[inline(always)]
    pub(crate) fn pixel(&self, at: Point2D<u16, PixelSpace>) -> bool {
        let untranslated = self.drawfrom_rect.origin + at.to_vector();
        let buffer_ix = untranslated.y as usize * self.overall_size.width as usize + untranslated.x as usize;

        let bit = self.buf[buffer_ix / 8] & (1 << (7 - buffer_ix % 8));
        return bit != 0;
    }
}