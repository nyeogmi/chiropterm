use std::collections::VecDeque;

use crate::aliases::*;
use crate::Interactor;
use super::{MouseButton, MouseEvent};

#[derive(Clone, Copy)]
pub struct DragMonitor {
    start: Option<State>,
    old: Option<State>,
}

#[derive(Clone, Copy)]
pub struct State {
    point: CellPoint,
}

impl DragMonitor {
    pub fn new() -> DragMonitor {
        DragMonitor {
            start: None,
            old: None,
        }
    }

    pub(crate) fn down(&mut self, point: CellPoint) {
        self.start = Some(State { point });
        self.old = self.start
    }

    pub(crate) fn at(
        &mut self, 
        events: &mut VecDeque<crate::MouseEvent>, 
        mouse_button: MouseButton,
        point: CellPoint, 
        interactor: &impl Fn(CellPoint) -> Interactor,
    ) {
        if self.start.is_none() { return; }
        let start = self.start.unwrap();
        let old = self.old.unwrap();  // set when start is set
        let new = State { point };

        if old.point == new.point { return }

        let start_interactor = interactor(start.point);
        let last_interactor = interactor(old.point);
        let new_interactor = interactor(new.point);

        events.push_back(MouseEvent::Drag {
            mouse_button,
            start_point: start.point,
            start_interactor,
            last_point: old.point,
            last_interactor: last_interactor,
            now_point: new.point,
            now_interactor: new_interactor
        });
        self.old = Some(new);
    }

    pub(crate) fn up(
        &mut self
    ) {
        self.start = None;
        self.old = None;
    }
}
