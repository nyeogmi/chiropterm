mod drag;
mod scroll_wheel;

use std::collections::VecDeque;

use enum_map::EnumMap;
use euclid::{point2};
use minifb::{MouseButton as MinifbMouseButton, MouseMode, Window};

use crate::{aliases::CellPoint, rendering::Interactor};

use self::scroll_wheel::ScrollWheelMonitor;

use super::{Aspect, input::MouseEvent, input::MouseButton};

use drag::DragMonitor;


pub(crate) struct Mouse {
    drag: EnumMap<MouseButton, DragMonitor>,
    scroll_wheel: ScrollWheelMonitor,

    old: Option<State>,
    new: Option<State>,
    events: VecDeque<MouseEvent>,
}


#[derive(Clone, Copy)]
struct State {
    down: EnumMap<MouseButton, bool>,

    cell_xy: CellPoint,
    interactor: Interactor,
}


impl Mouse {
    pub fn new() -> Mouse {
        Mouse { 
            drag: enum_map::enum_map! {
                _ => DragMonitor::new(),
            },
            scroll_wheel: ScrollWheelMonitor::new(),
            old: None, 
            new: None,
            events: VecDeque::new(),
        }
    }

    pub fn getch(&mut self) -> Option<MouseEvent> {
        self.events.pop_front()
    }

    // any_interactor: (normal, scroll)
    pub fn update(&mut self, aspect: Aspect, window: &mut Window, new_tick: bool, any_interactor: impl Fn(CellPoint) -> (Interactor, Interactor)) {
        let current_state = Mouse::current_state(aspect, window, &any_interactor);

        if let None = current_state {
            // don't bother generating events for now
            return;
        }

        self.old = self.new;
        self.new = current_state;

        use MouseEvent::*;

        match (self.old, self.new) {
            (None, None) => {}
            (None, Some(_)) => {}
            (Some(_), None) => {}
            (Some(old), Some(new)) => {
                if let Some((_, scroll_y)) = window.get_scroll_wheel() {
                    self.scroll_wheel.at(&mut self.events, new.cell_xy, scroll_y, &|p| any_interactor(p).1);
                }

                for mb in MouseButton::ALL {
                    if new.down[mb] && !old.down[mb] {
                        self.events.push_back(Click(mb, new.cell_xy, new.interactor));
                        self.drag[mb].down(new.cell_xy);
                    }

                    self.drag[mb].at(new.cell_xy);

                    if !new.down[mb] && old.down[mb] {
                        self.events.push_back(Up(mb, new.cell_xy, new.interactor));
                        self.drag[mb].up(&mut self.events, mb, &|p| any_interactor(p).0)  // TODO: Maybe just do this whenever !new.down?
                    }

                    if new_tick {
                        self.drag[mb].post_events(&mut self.events, mb, &|p| any_interactor(p).0)
                    }
                }
            }
        }
    }

    // normal interactor, scroll itneractor
    fn current_state(aspect: Aspect, window: &mut Window, get_interactor: &impl Fn(CellPoint) -> (Interactor, Interactor)) -> Option<State> {
        // NYEO NOTE: The logic in minifb to compensate for DPI scaling is wrong.
        // This logic is correct, however.
        let mouse_pos = if let Some(mp) = window.get_unscaled_mouse_pos(MouseMode::Pass) { 
            mp 
        } else { return None };
        let overall_size = window.get_size();
        let mouse_x_ideal = ((mouse_pos.0 / overall_size.0 as f32) * aspect.term_size.width as f32) as isize;
        let mouse_y_ideal = ((mouse_pos.1 / overall_size.1 as f32) * aspect.term_size.height as f32) as isize;
        
        let cell_xy = point2(mouse_x_ideal, mouse_y_ideal);

        let interactors = get_interactor(cell_xy);

        Some(State { 
            down: enum_map::enum_map![
                MouseButton::Left => window.get_mouse_down(MinifbMouseButton::Left),
                MouseButton::Right => window.get_mouse_down(MinifbMouseButton::Right),
            ],
            cell_xy,
            interactor: interactors.0,
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