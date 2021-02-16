use std::sync::Mutex;
use std::collections::HashSet;
use lazy_static::lazy_static;
use winit::event::WindowEvent;
use winit::event::*;

lazy_static! {
    /// The application's input singleton.
    ///
    /// # Example
    ///
    /// ```
    /// let mut input = INPUT.lock().unwrap();
    /// *input.clear_keys();
    /// ```
    pub static ref INPUT: Mutex<Input> = Mutex::new(Input::new());
}

pub struct Input {
    pressed_keys: HashSet<PKeyCode>,
    released_keys: HashSet<PKeyCode>,
}

impl Input {
    fn new() -> Self {
        Self {
            pressed_keys: HashSet::new(),
            released_keys: HashSet::new(),
        }
    }

    pub fn update(&mut self, event: &WindowEvent) -> bool{
        self.pressed_keys.clear();
        self.released_keys.clear();
        
        let mut was_input_event = true;
        
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(key_code),
                        ..
                    },
                ..
            } => {
                match key_code {
                    VirtualKeyCode::A => self.add_key_input(PKeyCode::A, state),
                    VirtualKeyCode::B => self.add_key_input(PKeyCode::B, state),
                    VirtualKeyCode::C => self.add_key_input(PKeyCode::C, state),
                    VirtualKeyCode::D => self.add_key_input(PKeyCode::D, state),
                    VirtualKeyCode::E => self.add_key_input(PKeyCode::E, state),
                    VirtualKeyCode::F => self.add_key_input(PKeyCode::F, state),
                    VirtualKeyCode::G => self.add_key_input(PKeyCode::G, state),
                    VirtualKeyCode::H => self.add_key_input(PKeyCode::H, state),
                    VirtualKeyCode::I => self.add_key_input(PKeyCode::I, state),
                    VirtualKeyCode::J => self.add_key_input(PKeyCode::J, state),
                    VirtualKeyCode::K => self.add_key_input(PKeyCode::K, state),
                    VirtualKeyCode::L => self.add_key_input(PKeyCode::L, state),
                    VirtualKeyCode::M => self.add_key_input(PKeyCode::M, state),
                    VirtualKeyCode::N => self.add_key_input(PKeyCode::N, state),
                    VirtualKeyCode::O => self.add_key_input(PKeyCode::O, state),
                    VirtualKeyCode::P => self.add_key_input(PKeyCode::P, state),
                    VirtualKeyCode::Q => self.add_key_input(PKeyCode::Q, state),
                    VirtualKeyCode::R => self.add_key_input(PKeyCode::R, state),
                    VirtualKeyCode::S => self.add_key_input(PKeyCode::S, state),
                    VirtualKeyCode::T => self.add_key_input(PKeyCode::T, state),
                    VirtualKeyCode::U => self.add_key_input(PKeyCode::U, state),
                    VirtualKeyCode::V => self.add_key_input(PKeyCode::V, state),
                    VirtualKeyCode::W => self.add_key_input(PKeyCode::W, state),
                    VirtualKeyCode::X => self.add_key_input(PKeyCode::X, state),
                    VirtualKeyCode::Y => self.add_key_input(PKeyCode::Y, state),
                    VirtualKeyCode::Z => self.add_key_input(PKeyCode::Z, state),
                    VirtualKeyCode::Key0 => self.add_key_input(PKeyCode::Key0, state),
                    VirtualKeyCode::Key1 => self.add_key_input(PKeyCode::Key1, state),
                    VirtualKeyCode::Key2 => self.add_key_input(PKeyCode::Key2, state),
                    VirtualKeyCode::Key3 => self.add_key_input(PKeyCode::Key3, state),
                    VirtualKeyCode::Key4 => self.add_key_input(PKeyCode::Key4, state),
                    VirtualKeyCode::Key5 => self.add_key_input(PKeyCode::Key5, state),
                    VirtualKeyCode::Key6 => self.add_key_input(PKeyCode::Key6, state),
                    VirtualKeyCode::Key7 => self.add_key_input(PKeyCode::Key7, state),
                    VirtualKeyCode::Key8 => self.add_key_input(PKeyCode::Key8, state),
                    VirtualKeyCode::Key9 => self.add_key_input(PKeyCode::Key9, state),
                    VirtualKeyCode::Up => self.add_key_input(PKeyCode::Up, state),
                    VirtualKeyCode::Down => self.add_key_input(PKeyCode::Down, state),
                    VirtualKeyCode::Left => self.add_key_input(PKeyCode::Left, state),
                    VirtualKeyCode::Right => self.add_key_input(PKeyCode::Right, state),
                    VirtualKeyCode::Return => self.add_key_input(PKeyCode::Return, state),
                    VirtualKeyCode::Space => self.add_key_input(PKeyCode::Space, state),
                    VirtualKeyCode::Tab => self.add_key_input(PKeyCode::Tab, state),
                    VirtualKeyCode::Escape => self.add_key_input(PKeyCode::Escape, state),
                    _ => was_input_event = false,
                }
            },
            _ => was_input_event = false,
        }

        was_input_event

    }

    pub fn add_key_input(&mut self, code: PKeyCode, state: &ElementState) {
        match state {
            ElementState::Pressed => {
                self.pressed_keys.insert(code);
                //println!("{:?}", code);
                ()
            },
            ElementState::Released => {
                self.released_keys.insert(code);
                ()
            },
        }
    }

    pub fn get_key_pressed(&mut self, key_code: PKeyCode) -> bool {
        self.pressed_keys.contains(&key_code)
    }
}

/// [`PKeyCode`] is a wrapper class to remove the specific
/// window event handling dependecies (such as [winit]) in
/// case of dependency migration at a later date. Therefore
/// any Input handling in applications that use [Palon] will
/// not break in the event of a migration.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum PKeyCode {
    // Numerical keys
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    // Alphabet
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    //
    Up,
    Down,
    Left,
    Right,
    Return,
    Space,
    Tab,
    Escape,
    // Modifiers
    LShift,
    RShift,
    LCtrl,
    RCtrl,
    LAlt,
    RAlt,
}

