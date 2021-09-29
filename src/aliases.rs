use euclid::{Point2D, Rect, Size2D, Vector2D};

// as a convenience thing for internal users
pub(crate) use euclid::{point2, vec2, rect};

pub(crate) struct PixelSpace;
// pub(crate) type PixelPoint = Point2D<u16, PixelSpace>;
// pub(crate) type PixelVector = Vector2D<u16, PixelSpace>;
pub(crate) type PixelSize = Size2D<u16, PixelSpace>;
// pub(crate) type PixelRect = Rect<u16, PixelSpace>;

// pub(crate) type PixelPointF32 = Point2D<f32, PixelSpace>;
// pub(crate) type PixelVectorF32 = Vector2D<f32, PixelSpace>;
pub(crate) type PixelSizeF32 = Size2D<f32, PixelSpace>;
// pub(crate) type PixelRectF32 = Rect<f32, PixelSpace>;

pub struct CellSpace;
pub type CellPoint = Point2D<isize, CellSpace>;
pub type CellVector = Vector2D<isize, CellSpace>;
pub type CellSize = Size2D<isize, CellSpace>;
pub type CellRect = Rect<isize, CellSpace>;

pub(crate) type CellPointU16 = Point2D<u16, CellSpace>;
// pub(crate) type CellVectorU16 = Vector2D<u16, CellSpace>;
pub(crate) type CellSizeU16 = Size2D<u16, CellSpace>;
pub(crate) type CellRectU16 = Rect<u16, CellSpace>;

// pub(crate) type CellPointF32 = Point2D<f32, CellSpace>;
// pub(crate) type CellVectorF32 = Vector2D<f32, CellSpace>;
pub(crate) type CellSizeF32 = Size2D<f32, CellSpace>;
// pub(crate) type CellRectF32 = Rect<f32, CellSpace>;