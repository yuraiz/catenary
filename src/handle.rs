use nannou::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HandleState {
    Still,
    Hover,
    Selected,
}

#[derive(Debug)]
pub struct Handle {
    pos: Point2,
    radius: f32,
    color: Rgb<u8>,
    state: HandleState,
}

const STILL_HANDLE_RADIUS: f32 = 6.0;
const HOVER_HANDLE_RADIUS: f32 = 8.0;

impl Handle {
    pub fn new(pos: Point2, color: Rgb<u8>) -> Self {
        Self {
            pos,
            color,
            radius: STILL_HANDLE_RADIUS,
            state: HandleState::Still,
        }
    }

    pub fn pos(&self) -> Point2 {
        self.pos
    }

    pub fn set_pos(&mut self, pos: Point2) {
        self.pos = pos;
    }

    pub fn distance(&self, pos: Point2) -> f32 {
        self.pos.distance(pos)
    }

    pub fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .radius(self.radius)
            .xy(self.pos)
            .color(self.color);
    }

    pub fn apply_mouse_moved(&mut self, pos: Point2, pressed: bool) {
        let hovered = self.pos.distance(pos) <= HOVER_HANDLE_RADIUS;

        self.state = match self.state {
            HandleState::Selected | HandleState::Hover if pressed => HandleState::Selected,
            _ => {
                if hovered && !pressed {
                    HandleState::Hover
                } else {
                    HandleState::Still
                }
            }
        };

        self.radius = if self.state == HandleState::Still {
            STILL_HANDLE_RADIUS
        } else {
            HOVER_HANDLE_RADIUS
        };

        if self.state == HandleState::Selected {
            self.pos = pos;
        }
    }

    pub fn is_selected(&self) -> bool {
        self.state == HandleState::Selected
    }
}
