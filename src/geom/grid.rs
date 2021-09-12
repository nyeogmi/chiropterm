// vendored from `gridd` package
// rewritten to use euclid types
use euclid::{Point2D, Rect, Size2D, point2, size2};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Grid<T, Space> {
    size: Size2D<usize, Space>,
    data: Vec<T>,
}

impl<T, Space> Grid<T, Space>
where
    T: Copy,
{
    pub fn new(size: Size2D<usize, Space>, default: T) -> Self {
        let capacity = size.width * size.height;

        Self { size, data: vec![default; capacity] }
    }

    pub fn square(side_len: usize, default: T) -> Self {
        Self::new(Size2D::new(side_len, side_len), default)
    }

    pub fn transpose(&self) -> Self {
        if let Some(&val) = self.get(point2(0, 0)) {
            let mut new_grid = Self::new(Size2D::new(self.size.height, self.size.width), val);

            for src_row in 0..self.size.width {
                for src_col in 0..self.size.height {
                    if let Some(&val) = self.get(point2(src_col, src_row)) {
                        new_grid.set(point2(src_row, src_col), val)
                    }
                }
            }

            new_grid
        } else {
            Self {
                size: size2(0, 0),
                data: Vec::new(),
            }
        }
    }
}

impl<T, Space> Grid<T, Space> {
    fn flat_index(&self, p: Point2D<usize, Space>) -> usize {
        p.x + self.size.width * p.y
    }

    pub fn rect(&self) -> Rect<usize, Space> {
        Rect::new(Point2D::zero(), self.size())
    }

    pub fn size(&self) -> Size2D<usize, Space> {
        self.size()
    }

    pub fn get(&self, p: Point2D<usize, Space>) -> Option<&T> {
        if self.contains(p) {
            let index = self.flat_index(p);

            Some(&self.data[index])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, p: Point2D<usize, Space>) -> Option<&mut T> {
        if self.contains(p) {
            let index = self.flat_index(p);

            Some(&mut self.data[index])
        } else {
            None
        }
    }

    pub fn set(&mut self, p: Point2D<usize, Space>, new_val: T) {
        match self.get_mut(p) {
            Some(val) => {
                *val = new_val;
            }
            None => (),
        }
    }

    /// Determine if a coordinate is within the grid
    pub fn contains(&self, p: Point2D<usize, Space>) -> bool {
        p.x < self.size.width && p.y < self.size.height
    }
}