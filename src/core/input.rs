use winit::event::{ScanCode, VirtualKeyCode};

#[derive(Default, Clone)]
pub struct KeyboardInput {
    p_keys: Vec<ScanCode>,
    v_keys: Vec<VirtualKeyCode>,
}

impl KeyboardInput {
    pub fn push_pkey(&mut self, key: ScanCode) { self.p_keys.push(key); }
    pub fn push_vkey(&mut self, key: VirtualKeyCode) { self.v_keys.push(key); }
    pub fn physical_keys(&self) -> &Vec<ScanCode> { &self.p_keys }
    pub fn virtual_keys(&self) -> &Vec<VirtualKeyCode> { &self.v_keys }

    pub fn clear_keys(&mut self) {
        self.p_keys.clear();
        self.v_keys.clear();
    }
}

#[derive(Default, Clone)]
pub struct Input {
    pub keyboard: KeyboardInput,
    //mouse
}

impl Input {
    pub fn process_event<'a>(&mut self, event: &winit::event::Event<'a, ()>) {
        match event {
            winit::event::Event::WindowEvent { window_id, event } => match event {
                winit::event::WindowEvent::KeyboardInput {
                    device_id,
                    input,
                    is_synthetic,
                } => {
                    if let Some(key) = input.virtual_keycode {
                        self.keyboard.push_vkey(key);
                    }
                    self.keyboard.push_pkey(input.scancode);
                }
                // winit::event::WindowEvent::CursorMoved { device_id, position, modifiers } => todo!(),
                // winit::event::WindowEvent::CursorEntered { device_id } => todo!(),
                // winit::event::WindowEvent::CursorLeft { device_id } => todo!(),
                // winit::event::WindowEvent::MouseWheel { device_id, delta, phase, modifiers } => todo!(),
                // winit::event::WindowEvent::MouseInput { device_id, state, button, modifiers } => todo!(),
                _ => {}
            },
            _ => {}
        }
    }
}
