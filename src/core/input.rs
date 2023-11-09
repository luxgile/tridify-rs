use winit::event::{ScanCode, VirtualKeyCode};

#[derive(PartialEq, Clone)]
pub struct Key {
    pub vkey: VirtualKeyCode,
    pub just_pressed: bool,
    frame_press: bool,
}
impl Key {
    pub fn new(key: VirtualKeyCode) -> Self {
        Self {
            vkey: key,
            just_pressed: true,
            frame_press: true,
        }
    }
}

#[derive(Default, Clone)]
pub struct KeyboardInput {
    p_keys: Vec<ScanCode>,
    v_keys: Vec<Key>,
}

impl KeyboardInput {
    pub fn push_pkey(&mut self, key: ScanCode) { self.p_keys.push(key); }
    pub fn push_vkey(&mut self, key: Key) {
        let index = self.v_keys.iter().position(|x| *x == key);
        if index.is_none() {
            self.v_keys.push(key);
        } else {
            self.v_keys[index.unwrap()].just_pressed = false;
            self.v_keys[index.unwrap()].frame_press = true;
        }
    }
    pub fn release_vkey(&mut self, key: Key) {
        let index = self.v_keys.iter().position(|x| *x == key);
        if index.is_some() {
            self.v_keys.remove(index.unwrap());
        }
    }

    pub fn physical_keys(&self) -> &Vec<ScanCode> { &self.p_keys }
    pub fn virtual_keys(&self) -> &Vec<Key> { &self.v_keys }

    pub fn clear_keys(&mut self) {
        self.p_keys.clear();
        self.v_keys = self
            .v_keys
            .drain(..)
            .filter_map(|mut x| {
                if x.frame_press {
                    x.frame_press = false;
                    Some(x)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
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
                        let key = Key::new(key);
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
