// vendored from `gridd` package
// rewritten to use euclid types
// and rewritten to have a customizable rectangle boundary
// and to remove some unnecessary helpers (ex: transpose(), square())
use euclid::{Point2D, Rect, Size2D};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Grid<T, Space> {
    rect: Rect<isize, Space>,
    data: Vec<T>,
}

impl<T, Space> Grid<T, Space>
where
    T: Copy,
{
    pub fn new(rect: Rect<isize, Space>, default: T) -> Self {
        assert!(rect.size.width >= 0);
        assert!(rect.size.height >= 0);

        let capacity = rect.size.width as usize * rect.size.height as usize;
        Self { rect, data: vec![default; capacity] }
    }
}

impl<T, Space> Grid<T, Space> {
    pub fn contains(&self, p: Point2D<isize, Space>) -> bool {
        self.rect.contains(p)
    }

    fn flat_index(&self, p: Point2D<isize, Space>) -> usize {
        assert!(self.rect.contains(p));

        let w = self.rect.size.width as usize;
        let y = (p.y - self.rect.origin.y) as usize;
        let x = (p.x - self.rect.origin.x) as usize;
        w * y + x
    }

    pub fn rect(&self) -> Rect<isize, Space> {
        self.rect
    }

    pub fn size(&self) -> Size2D<isize, Space> {
        self.rect.size
    }

    pub fn get(&self, p: Point2D<isize, Space>) -> Option<&T> {
        if self.contains(p) {
            let index = self.flat_index(p);

            Some(&self.data[index])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, p: Point2D<isize, Space>) -> Option<&mut T> {
        if self.contains(p) {
            let index = self.flat_index(p);

            Some(&mut self.data[index])
        } else {
            None
        }
    }

    pub fn set(&mut self, p: Point2D<isize, Space>, new_val: T) {
        match self.get_mut(p) {
            Some(val) => {
                *val = new_val;
            }
            None => (),
        }
    }
}