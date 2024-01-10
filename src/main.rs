mod chain;
mod handle;

use chain::Chain;
use handle::Handle;

use nannou::prelude::*;

fn main() {
    nannou::app(Model::new)
        .event(event)
        .simple_window(view)
        .run();
}

#[derive(Debug)]
struct Model {
    handles: Vec<Handle>,
    chains: Vec<Chain>,
    mouse_pressed: bool,
}

impl Model {
    fn new(_: &App) -> Self {
        Self {
            handles: vec![
                Handle::new(pt2(-100.0, 0.0), Rgb::new(255, 150, 150)),
                Handle::new(pt2(100.0, 0.0), Rgb::new(150, 150, 255)),
            ],
            chains: vec![Chain::new(0, 1, 300.0)],
            mouse_pressed: false,
        }
    }
}

fn event(_app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent {
            simple: Some(event),
            ..
        } => match event {
            MouseMoved(pos) => {
                let handle =
                    if let Some(selected) = model.handles.iter_mut().find(|h| h.is_selected()) {
                        selected
                    } else if let Some(closest) = model.handles.iter_mut().min_by(|a, b| {
                        let a = a.distance(pos);
                        let b = b.distance(pos);
                        a.partial_cmp(&b).unwrap()
                    }) {
                        closest
                    } else {
                        return;
                    };

                handle.apply_mouse_moved(pos, model.mouse_pressed);
            }
            MousePressed(MouseButton::Left) => model.mouse_pressed = true,
            MouseReleased(MouseButton::Left) => model.mouse_pressed = false,
            _ => {}
        },
        Event::Update(upd) => model.chains.iter().for_each(|chain| {
            chain.update_physics(&mut model.handles, upd.since_last.as_secs_f32())
        }),
        _ => {}
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);

    let draw = app.draw();

    model
        .chains
        .iter()
        .for_each(|c| c.draw(&model.handles, &draw));

    model.handles.iter().for_each(|h| h.draw(&draw));

    draw.to_frame(app, &frame).unwrap();
}
