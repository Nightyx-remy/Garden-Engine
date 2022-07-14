pub use glfw::MouseButtonLeft;
pub use glfw::MouseButtonMiddle;
pub use glfw::MouseButtonRight;
pub use glfw::MouseButton;
pub use glfw::Key;

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                             Action                                             //
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Action {
    Pressed,
    Down,
    Released,
    Up
}

impl Action {

    fn update(&mut self) {
        match self {
            Action::Pressed => *self = Action::Down,
            Action::Released => *self = Action::Up,
            _ => {}
        }
    }

    pub fn is_pressed(&self) -> bool {
        return *self == Action::Pressed;
    }

    pub fn is_down(&self) -> bool {
        return *self == Action::Down;
    }

    pub fn is_released(&self) -> bool {
        return *self == Action::Released;
    }

    pub fn is_up(&self) -> bool {
        return *self == Action::Up;
    }

    pub fn is_pressed_or_down(&self) -> bool {
        return self.is_pressed() || self.is_down();
    }

    pub fn is_pressed_or_released(&self) -> bool {
        return self.is_pressed() || self.is_released();
    }

    pub fn is_pressed_or_up(&self) -> bool {
        return self.is_pressed() || self.is_up();
    }

    pub fn is_down_or_released(&self) -> bool {
        return self.is_down() || self.is_released();
    }

    pub fn is_down_or_up(&self) -> bool {
        return self.is_down() || self.is_up();
    }

    pub fn is_released_or_up(&self) -> bool {
        return self.is_released() || self.is_up();
    }

}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                              Mouse                                             //
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Mouse {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) last_x: i32,
    pub(crate) last_y: i32,
    pub(crate) scroll_x: f64,
    pub(crate) scroll_y: f64,
    pub(crate) buttons: [Action; (glfw::ffi::MOUSE_BUTTON_LAST + 1) as usize],
}

impl Mouse {

    fn new() -> Mouse {
        return Mouse {
            x: 0,
            y: 0,
            last_x: 0,
            last_y: 0,
            scroll_x: 0.0,
            scroll_y: 0.0,
            buttons: [Action::Up; (glfw::ffi::MOUSE_BUTTON_LAST + 1) as usize],
        }
    }

    fn update(&mut self) {
        self.last_x = self.x;
        self.last_y = self.y;
        self.scroll_x = 0.0;
        self.scroll_y = 0.0;

        self.buttons.iter_mut().for_each(|button| button.update());
    }

    // Getters
    pub fn get_x(&self) -> i32 {
        return self.x;
    }

    pub fn get_y(&self) -> i32 {
        return self.y;
    }

    pub fn get_last_x(&self) -> i32 {
        return self.last_x;
    }

    pub fn get_last_y(&self) -> i32 {
        return self.last_y;
    }

    pub fn get_dx(&self) -> i32 {
        return self.last_x - self.x;
    }

    pub fn get_dy(&self) -> i32 {
        return self.last_y - self.y;
    }

    pub fn get_scroll_x(&self) -> f64 {
        return self.scroll_x;
    }

    pub fn is_scroll_left(&self) -> bool {
        return self.scroll_x > 0.0;
    }

    pub fn is_scroll_right(&self) -> bool {
        return self.scroll_x < 0.0;
    }
    
    pub fn get_scroll_y(&self) -> f64 {
        return self.scroll_y;
    }
    
    pub fn is_scroll_up(&self) -> bool {
        return self.scroll_y > 0.0;
    }
    
    pub fn is_scroll_down(&self) -> bool {
        return self.scroll_y < 0.0;
    }

    pub fn get_button(&self, button: MouseButton) -> Action {
        return self.buttons[button as usize];
    }

}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                            Keyboard                                            //
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Keyboard {
    pub(crate) keys: [Action; (glfw::ffi::KEY_LAST + 1) as usize]
}

impl Keyboard {

    pub fn new() -> Keyboard {
        return Keyboard {
            keys: [Action::Up; (glfw::ffi::KEY_LAST + 1) as usize]
        }
    }

    pub fn update(&mut self) {
        self.keys.iter_mut().for_each(|key| key.update());
    }

    pub fn get_key(&self, key: Key) -> Action {
        return self.keys[key as usize];
    }

}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                              Input                                             //
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Input {
    pub(crate) mouse: Mouse,
    pub(crate) keyboard: Keyboard,
}

impl Input {

    pub(crate) fn new() -> Input {
        return Input {
            mouse: Mouse::new(),
            keyboard: Keyboard::new(),
        }
    }

    pub(crate) fn update(&mut self) {
        self.mouse.update();
        self.keyboard.update();
    }

    pub fn mouse(&self) -> &Mouse {
        return &self.mouse;
    }

    pub fn keyboard(&self) -> &Keyboard {
        return &self.keyboard;
    }

}