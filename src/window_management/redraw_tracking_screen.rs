use std::mem;

use crate::{CellSize, Screen, constants::{CELL_X, CELL_Y}, rendering::Render};

use gridd_euclid::PointsIn;

pub(crate) struct RedrawTrackingScreen {
    old: Screen,
    new_frame: u64,
    new: Screen,

    last_frame_drawn: u64,
    last_render: Option<Render>,
}

impl RedrawTrackingScreen {
    pub(crate) fn new(default_bg: u8, default_fg: u8) -> RedrawTrackingScreen {
        RedrawTrackingScreen {
            old: Screen::new(default_bg, default_fg),
            new_frame: 1,
            new: Screen::new(default_bg, default_fg),

            last_frame_drawn: 0,
            last_render: None,
        }
    }

    pub(crate) fn target(&self) -> &crate::Screen {
        &self.new
    }

    pub(crate) fn resize(&mut self, size: CellSize) {
        self.new.resize(size);
    }

    pub(crate) fn switch(&mut self) {
        let size = self.new.rect().size;
        mem::swap(&mut self.old, &mut self.new);
        self.new_frame += 1;
        self.new.clear();
        self.new.resize(size);  
    }

    fn old_frame(&self) -> u64 {
        self.new_frame - 1
    }

    // bool: whether any changes were made
    pub fn draw(&mut self, render: Render, buffer: &mut Vec<u32>) -> bool {
        let new_render = render;

        let (last_screen, new_screen) = if self.last_frame_drawn == self.old_frame() {
            (&self.old, &self.new)
        } else if self.last_frame_drawn == self.new_frame {
            if self.last_render.as_ref() == Some(&new_render) {
                return false;
            }
            (&self.new, &self.new)
        } else {
            self.completely_redraw(buffer, &new_render, &self.new);
            self.last_frame_drawn = self.new_frame;
            self.last_render.replace(new_render);
            return true;
        };

        let touched  = match &self.last_render {
            Some(last_render) if last_render.aspect == new_render.aspect => {
                self.draw_differences(buffer, last_render, &last_screen, &new_render, new_screen)
            }
            _ => {
                self.completely_redraw(buffer, &new_render, &self.new);
                true
            }
        };

        self.last_frame_drawn = self.new_frame;
        self.last_render.replace(new_render);
        return touched;
    }

    fn draw_differences(
        &self, 
        buffer: &mut Vec<u32>,
        last_render: &Render, last_screen: &Screen, new_render: &Render, new_screen: &Screen,
    ) -> bool {
        let screen_rect = new_screen.rect();
        let term_rect = new_render.aspect.term_rect();
        assert_eq!(screen_rect, term_rect.cast());
        assert_eq!(screen_rect.area() as usize * CELL_X * CELL_Y, buffer.len());

        let mut touched = false;
        for term_xy in u16::points_in(term_rect) {
            let old_content = last_render.get_content(last_screen, term_xy.cast());
            let new_content = new_render.get_content(new_screen, term_xy.cast());
            if old_content != new_content {
                new_content.physically_draw(
                    buffer, term_xy.x, term_xy.y, new_render.aspect.term_size.width, 
                );
                touched = true
            }
        }
        touched
    }

    fn completely_redraw(
        &self, 
        buffer: &mut Vec<u32>, 
        render: &Render, screen: &Screen
    ) {
        let screen_rect = screen.rect();
        let term_rect = render.aspect.term_rect();
        assert_eq!(screen_rect, term_rect.cast());
        assert_eq!(screen_rect.area() as usize * CELL_X * CELL_Y, buffer.len());

        for term_xy in u16::points_in(term_rect) {
            render.get_content(screen, term_xy.cast()).physically_draw(
                buffer, term_xy.x, term_xy.y, render.aspect.term_size.width, 
            );
        }
    }
}