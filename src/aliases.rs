use euclid::{Point2D, Rect, Size2D, Vector2D};

pub use euclid::{size2, point2, vec2, rect};

pub struct PixelSpace;
pub type PixelPoint = Point2D<u16, PixelSpace>;
pub type PixelVector = Vector2D<u16, PixelSpace>;
pub type PixelSize = Size2D<u16, PixelSpace>;
pub type PixelRect = Rect<u16, PixelSpace>;

pub type PixelPointF32 = Point2D<f32, PixelSpace>;
pub type PixelVectorF32 = Vector2D<f32, PixelSpace>;
pub type PixelSizeF32 = Size2D<f32, PixelSpace>;
pub type PixelRectF32 = Rect<f32, PixelSpace>;

pub struct CellSpace;
pub type CellPoint = Point2D<isize, CellSpace>;
pub type CellVector = Vector2D<isize, CellSpace>;
pub type CellSize = Size2D<isize, CellSpace>;
pub type CellRect = Rect<isize, CellSpace>;

pub type CellPointU16 = Point2D<u16, CellSpace>;
pub type CellVectorU16 = Vector2D<u16, CellSpace>;
pub type CellSizeU16 = Size2D<u16, CellSpace>;
pub type CellRectU16 = Rect<u16, CellSpace>;

pub type CellPointF32 = Point2D<f32, CellSpace>;
pub type CellVectorF32 = Vector2D<f32, CellSpace>;
pub type CellSizeF32 = Size2D<f32, CellSpace>;
pub type CellRectF32 = Rect<f32, CellSpace>;