use std::collections::VecDeque;

use crate::aliases::*;
use crate::Interactor;
use super::{MouseButton, MouseEvent};

#[derive(Clone, Copy)]
pub struct DragMonitor {
    start: Option<State>,
    old: Option<State>,
    event_to_send: Option<ToSend>, 
}

#[derive(Clone, Copy)]
struct ToSend {
    start: CellPoint,
    last: CellPoint,
    now: CellPoint,
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
            event_to_send: None,  // use this to rate-limit
        }
    }

    pub(crate) fn down(&mut self, point: CellPoint) {
        self.start = Some(State { point });
        self.old = self.start
    }

    pub(crate) fn at(
        &mut self, 
        point: CellPoint, 
    ) {
        if self.start.is_none() { return; }
        let start = self.start.unwrap();
        let old = self.old.unwrap();  // set when start is set
        let new = State { point };

        if old.point == new.point { return }

        if let Some(e) = &mut self.event_to_send {
            e.now = new.point;
        } else {
            self.event_to_send = Some(ToSend { start: start.point, last: old.point, now: new.point });
        }
        self.old = Some(new);
    }

    pub(crate) fn post_events(
        &mut self,
        events: &mut VecDeque<crate::MouseEvent>, 
        mouse_button: MouseButton,
        interactor: &impl Fn(CellPoint) -> Interactor,
    ) {
        if let Some(ToSend { start, last, now }) = self.event_to_send.take() {
            let start_interactor = interactor(start);
            let last_interactor = interactor(last);
            let now_interactor = interactor(now);

            events.push_back(MouseEvent::Drag {
                mouse_button,
                start_point: start,
                start_interactor,
                last_point: last,
                last_interactor: last_interactor,
                now_point: now,
                now_interactor: now_interactor
            });
        }
    }

    pub(crate) fn up(
        &mut self,
        events: &mut VecDeque<crate::MouseEvent>,
        mouse_button: MouseButton,
        interactor: &impl Fn(CellPoint) -> Interactor,
    ) {
        self.post_events(events, mouse_button, interactor);

        self.event_to_send = None;
        self.start = None;
        self.old = None;
    }
}
