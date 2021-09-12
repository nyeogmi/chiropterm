use euclid::{Point2D, Rect, point2};

pub trait PointsIn<X>: Sized {
    type T: DoubleEndedIterator<Item=Point2D<Self, X>>;
    fn points_in(r: Rect<Self, X>) -> Self::T;
}

impl<X> PointsIn<X> for u8 {
    type T = impl DoubleEndedIterator<Item=Point2D<Self, X>>;

    #[inline(always)]
    fn points_in(r: Rect<Self, X>) -> Self::T {
        r.y_range().flat_map(move |y| r.x_range().map(move |x| point2(x, y)))
    }
}

impl<X> PointsIn<X> for u16 {
    type T = impl DoubleEndedIterator<Item=Point2D<Self, X>>;

    #[inline(always)]
    fn points_in(r: Rect<Self, X>) -> Self::T {
        r.y_range().flat_map(move |y| r.x_range().map(move |x| point2(x, y)))
    }
}

impl<X> PointsIn<X> for u32 {
    type T = impl DoubleEndedIterator<Item=Point2D<Self, X>>;

    #[inline(always)]
    fn points_in(r: Rect<Self, X>) -> Self::T {
        r.y_range().flat_map(move |y| r.x_range().map(move |x| point2(x, y)))
    }
}

impl<X> PointsIn<X> for u64 {
    type T = impl DoubleEndedIterator<Item=Point2D<Self, X>>;

    #[inline(always)]
    fn points_in(r: Rect<Self, X>) -> Self::T {
        r.y_range().flat_map(move |y| r.x_range().map(move |x| point2(x, y)))
    }
}

impl<X> PointsIn<X> for usize {
    type T = impl DoubleEndedIterator<Item=Point2D<Self, X>>;

    #[inline(always)]
    fn points_in(r: Rect<Self, X>) -> Self::T {
        r.y_range().flat_map(move |y| r.x_range().map(move |x| point2(x, y)))
    }
}

impl<X> PointsIn<X> for i8 {
    type T = impl DoubleEndedIterator<Item=Point2D<Self, X>>;

    #[inline(always)]
    fn points_in(r: Rect<Self, X>) -> Self::T {
        r.y_range().flat_map(move |y| r.x_range().map(move |x| point2(x, y)))
    }
}

impl<X> PointsIn<X> for i16 {
    type T = impl DoubleEndedIterator<Item=Point2D<Self, X>>;

    #[inline(always)]
    fn points_in(r: Rect<Self, X>) -> Self::T {
        r.y_range().flat_map(move |y| r.x_range().map(move |x| point2(x, y)))
    }
}

impl<X> PointsIn<X> for i32 {
    type T = impl DoubleEndedIterator<Item=Point2D<Self, X>>;

    #[inline(always)]
    fn points_in(r: Rect<Self, X>) -> Self::T {
        r.y_range().flat_map(move |y| r.x_range().map(move |x| point2(x, y)))
    }
}

impl<X> PointsIn<X> for i64 {
    type T = impl DoubleEndedIterator<Item=Point2D<Self, X>>;

    #[inline(always)]
    fn points_in(r: Rect<Self, X>) -> Self::T {
        r.y_range().flat_map(move |y| r.x_range().map(move |x| point2(x, y)))
    }
}

impl<X> PointsIn<X> for isize {
    type T = impl DoubleEndedIterator<Item=Point2D<Self, X>>;

    #[inline(always)]
    fn points_in(r: Rect<Self, X>) -> Self::T {
        r.y_range().flat_map(move |y| r.x_range().map(move |x| point2(x, y)))
    }
}