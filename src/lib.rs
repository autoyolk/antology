use std::error::Error;

use winit::{
    event::*, event_loop::EventLoop, keyboard::{Key, NamedKey}, window::WindowBuilder
};

pub fn run() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new().build(&event_loop)?;

    event_loop.run(move |event, target| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event: KeyEvent {
                    state: ElementState::Pressed,
                    logical_key: Key::Named(NamedKey::Escape),
                    ..
                },
                ..
            } => target.exit(),
            _ => {}
        },
        _ => {}
    })?;

    Ok(())
}

