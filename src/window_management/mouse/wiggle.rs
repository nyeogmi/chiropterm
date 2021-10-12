use std::collections::VecDeque;

use crate::aliases::*;
use crate::Interactor;
use super::MouseEvent;

#[derive(Clone, Copy)]
pub struct WiggleMonitor {
    old: Option<State>,
    event_to_send: Option<ToSend>, 
}

#[derive(Clone, Copy)]
struct ToSend {
    last: CellPoint,
    now: CellPoint,
}

#[derive(Clone, Copy)]
pub struct State {
    point: CellPoint,
}

impl WiggleMonitor {
    pub fn new() -> WiggleMonitor {
        WiggleMonitor {
            old: None,
            event_to_send: None,  // use this to rate-limit
        }
    }

    pub(crate) fn at(
        &mut self, 
        point: CellPoint, 
    ) {
        let new = State { point };
        let old = self.old.take();
        self.old.replace(new);
        let old = if let Some(old) = old { old } else { return };

        if old.point == new.point { return }

        if let Some(e) = &mut self.event_to_send {
            e.now = new.point;
        } else {
            self.event_to_send = Some(ToSend { last: old.point, now: new.point });
        }
    }

    pub(crate) fn post_events(
        &mut self,
        events: &mut VecDeque<crate::MouseEvent>, 
        interactor: &impl Fn(CellPoint) -> Interactor,
    ) {
        if let Some(ToSend { last, now }) = self.event_to_send.take() {
            let last_interactor = interactor(last);
            let now_interactor = interactor(now);

            events.push_back(MouseEvent::Wiggle {
                last_point: last,
                last_interactor,
                now_point: now,
                now_interactor,
            });
        }
    }
}
