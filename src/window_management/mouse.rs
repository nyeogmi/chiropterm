use std::collections::VecDeque;

use euclid::{point2};
use minifb::{MouseButton as MinifbMouseButton, MouseMode, Window};

use crate::{aliases::CellPoint, rendering::Interactor};

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
    interactor: Interactor,
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

    pub fn update(&mut self, aspect: Aspect, window: &mut Window, any_interactor: impl Fn(CellPoint) -> Interactor) {
        let current_state = Mouse::current_state(aspect, window, any_interactor);

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
                if new.left && !old.left { self.events.push_back(Click(Left, new.cell_xy, new.interactor)) }
                if !new.left && old.left { self.events.push_back(Up(Left, new.cell_xy, new.interactor)) }
                if new.right && !old.right { self.events.push_back(Click(Right, new.cell_xy, new.interactor)) }
                if !new.right && old.right { self.events.push_back(Up(Right, new.cell_xy, new.interactor)) }
            }
        }
    }

    fn current_state(aspect: Aspect, window: &mut Window, get_interactor: impl Fn(CellPoint) -> Interactor) -> Option<State> {
        let mouse_pos = if let Some(mp) = window.get_mouse_pos(MouseMode::Pass) { 
            mp 
        } else { return None };
        let cell_xy = point2(
            (mouse_pos.0 / aspect.cell_size.width as f32) as isize, 
            (mouse_pos.1 / aspect.cell_size.height as f32) as isize,
        );

        Some(State { 
            left: window.get_mouse_down(MinifbMouseButton::Left),
            right: window.get_mouse_down(MinifbMouseButton::Right),
            cell_xy,
            interactor: get_interactor(cell_xy),
        })
    }

    pub fn interactor_changed(&self) -> bool {
        let i0 = Self::ext_interactor(self.old);
        let i1 = Self::ext_interactor(self.new);
        return i0 != i1;
    }

    pub fn interactor(&self) -> Interactor {
        Self::ext_interactor(self.new)
    }

    fn ext_interactor(st: Option<State>) -> Interactor {
        match st {
            None => Interactor::none(),
            Some(st) => st.interactor
        }
    }
}
// TODO: Scroll wheel?
// TODO: Drag events?