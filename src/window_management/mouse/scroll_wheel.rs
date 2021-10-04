use std::collections::VecDeque;

use crate::{CellPoint, Interactor, MouseEvent};

pub struct ScrollWheelMonitor {
}

impl ScrollWheelMonitor {
    pub fn new() -> ScrollWheelMonitor {
        ScrollWheelMonitor { }
    }

    pub(crate) fn at(
        &mut self, 
        events: &mut VecDeque<crate::MouseEvent>, 
        point: CellPoint, 
        scroll_y: f32,
        interactor: &impl Fn(CellPoint) -> Interactor,
    ) {
        // NOTE: Currently scroll_y is always divisible by 12
        events.push_back(MouseEvent::Scroll(-scroll_y / 12.0, point, interactor(point)));
    }
}
