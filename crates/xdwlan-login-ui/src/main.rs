#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use winit::event_loop::EventLoop;
use xdwlan_login_ui::app::{App, UserEvent};

#[cfg(target_os = "windows")]
fn main() -> anyhow::Result<()> {
    let event_loop = EventLoop::<UserEvent>::with_user_event().build()?;
    let mut app = App::new(&event_loop);

    App::register_events(&event_loop)?;
    event_loop.run_app(&mut app)?;

    Ok(())
}
