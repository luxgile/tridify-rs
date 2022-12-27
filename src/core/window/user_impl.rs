use glium::glutin::event::Event;

use crate::{LErr, Window};

/// Manages window lifetime events from the user side.
/// Needs to be implemented in user defined struct and sent to LDrawy to start drawing a window.
pub trait UserHandle<W: Window> {
    fn startup(&mut self, _wnd: &mut W) -> Result<(), LErr> { Ok(()) }

    fn process_logic(&mut self, _wnd: &mut W, _event: &Event<'_, ()>) -> Result<(), LErr> { Ok(()) }

    fn process_render(&mut self, _wnd: &mut W) -> Result<(), LErr> { Ok(()) }

    fn cleanup(&mut self, _wnd: &mut W) {
    }
}
