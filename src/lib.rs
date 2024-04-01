use anyhow::Result;
use winit::{
    event::*, event_loop::EventLoop, keyboard::{Key, NamedKey}, window::{Window, WindowBuilder}
};


pub fn init() -> Result<(EventLoop<()>, Window),> {
    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new().build(&event_loop)?;

    Ok((event_loop, window))
}

pub async fn run(event_loop: EventLoop<()>, window: Window) -> Result<()> {

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

