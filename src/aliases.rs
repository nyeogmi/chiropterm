use euclid::Point2D;

pub struct PixelSpace;
pub struct CellSpace;

pub type AtCell = Point2D<usize, CellSpace>;
pub type AtCellI = Point2D<isize, CellSpace>;