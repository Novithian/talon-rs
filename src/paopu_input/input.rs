//use winit::event::*;
use crate::core::{
    application::Application, module::Module,
};
use std::any::Any;

pub struct Input {}

impl Module for Input {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    // Needs a window to build properly
    fn build(&self, _app: &mut Application) {
        // TODO: After creating get module in ApplicationBuilder
        //let window_handle = &app.get_module::<Window>().unwrap().window;
        //self.setup(window_handle);
    }
}

/// [`TKeyCode`] is a wrapper class to remove the specific
/// window event handling dependecies (such as [winit]) in
/// case of dependency migration at a later date. Therefore
/// any Input handling in applications that use [Talon] will
/// not break in the event of a migration.
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum TKeyCode {
    // Numerical keys
    TKey0,
    TKey1,
    TKey2,
    TKey3,
    TKey4,
    TKey5,
    TKey6,
    TKey7,
    TKey8,
    TKey9,
    // Alphabet
    TKeyA,
    TKeyB,
    TKeyC,
    TKeyD,
    TKeyE,
    TKeyF,
    TKeyG,
    TKeyH,
    TKeyI,
    TKeyJ,
    TKeyK,
    TKeyL,
    TKeyM,
    TKeyN,
    TKeyO,
    TKeyP,
    TKeyQ,
    TKeyR,
    TKeyS,
    TKeyT,
    TKeyU,
    TKeyV,
    TKeyW,
    TKeyX,
    TKeyY,
    TKeyZ,
    //
    TKeyUp,
    TKeyDown,
    TKeyLeft,
    TKeyRight,
    TKeyReturn,
    TKeySpace,
    TKeyTab,
    TKeyEscape,
    // Modifiers
    TKeyLShift,
    TKeyRShift,
    TKeyLCtrl,
    TKeyRCtrl,
    TKeyLAlt,
    TKeyRAlt,
}

pub fn is_key_pressed(_key: TKeyCode) -> bool {
    false
}
