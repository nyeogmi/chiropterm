use std::collections::VecDeque;

use euclid::{point2};
use minifb::{MouseButton as MinifbMouseButton, MouseMode, Window};

use crate::aliases::CellPoint;

use super::{Aspect, input::MouseEvent, input::MouseButton};

pub struct Mouse {
    old: Option<State>,
    new: Option<State>,
    events: VecDeque<MouseEvent>,
}


#[derive(Clone, Copy)]
struct State {
    left: bool,
    right: bool,
    cell_xy: CellPoint,
}


impl Mouse {
    pub fn new() -> Mouse {
        Mouse { 
            old: None, 
            new: None,
            events: VecDeque::new(),
        }
    }

    pub fn getch(&mut self) -> Option<MouseEvent> {
        self.events.pop_front()
    }

    pub fn update(&mut self, aspect: Aspect, window: &mut Window) {
        let current_state = Mouse::current_state(aspect, window);

        if let None = current_state {
            // don't bother generating events for now
            return;
        }

        self.old = self.new;
        self.new = current_state;

        use MouseEvent::*;
        use MouseButton::*;
        match (self.old, self.new) {
            (None, None) => {}
            (None, Some(_)) => {}
            (Some(_), None) => {}
            (Some(old), Some(new)) => {
                if new.left && !old.left { self.events.push_back(Click(Left, new.cell_xy)) }
                if !new.left && old.left { self.events.push_back(Up(Left, new.cell_xy)) }
                if new.right && !old.right { self.events.push_back(Click(Right, new.cell_xy)) }
                if !new.right && old.right { self.events.push_back(Up(Right, new.cell_xy)) }
            }
        }
    }

    fn current_state(aspect: Aspect, window: &mut Window) -> Option<State> {
        let mouse_pos = if let Some(mp) = window.get_mouse_pos(MouseMode::Pass) { 
            mp 
        } else { return None };

        Some(State { 
            left: window.get_mouse_down(MinifbMouseButton::Left),
            right: window.get_mouse_down(MinifbMouseButton::Right),
            cell_xy: point2(
                (mouse_pos.0 / aspect.cell_size.width as f32) as isize, 
                (mouse_pos.1 / aspect.cell_size.height as f32) as isize,
            )
        })
    }
}
// TODO: Scroll wheel?
// TODO: Drag events?