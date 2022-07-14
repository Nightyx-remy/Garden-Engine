////////////////////////////////////////////////////////////////////////////////////////////////////
//                                          Configuration                                         //
////////////////////////////////////////////////////////////////////////////////////////////////////

use glfw::{Callback, Context, Error, SwapInterval, WindowEvent, WindowHint, WindowMode};
use std::sync::mpsc::Receiver;

use crate::{critical, error};

use super::input::{Action, Input};

pub struct WindowConfig<'a> {
    pub width: u32,
    pub height: u32,
    pub title: &'a str,
}

impl<'a> Default for WindowConfig<'a> {
    fn default() -> Self {
        return WindowConfig {
            width: 800,
            height: 600,
            title: "Title",
        };
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                             Window                                             //
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Window {
    window_ptr: glfw::Window,
    events: Receiver<(f64, WindowEvent)>,
    // Data
    width: u32,
    height: u32,
    scale_x: f32,
    scale_y: f32,
    x: i32,
    y: i32,
    // Input
    input: Input,
    // Flags
    size_changed: bool,
    pos_changed: bool,
}

impl Window {
    pub fn new(config: WindowConfig) -> Window {
        fn error_callback(error: Error, str: String, _: &()) {
            critical!("Tomato:GLFW", "{}, {}", error, str);
        }

        // Initialize GLFW
        let mut glfw = match glfw::init(Some(Callback {
            f: error_callback,
            data: (),
        })) {
            Ok(glfw) => glfw,
            Err(err) => critical!("Tomato:GLFW", "{}", err),
        };

        // Set Window hints
        glfw.window_hint(WindowHint::Visible(false));

        let (width, height) = (config.width, config.height);

        // Calculate the x and y coordinates to center the window
        let (x, y) = glfw.with_primary_monitor(|_, monitor| {
            if let Some(monitor) = monitor {
                if let Some(video_mode) = monitor.get_video_mode() {
                    (
                        ((video_mode.width - width) / 2) as i32,
                        ((video_mode.height - height) / 2) as i32,
                    )
                } else {
                    error!(
                        "Tomato:GLFW",
                        "Failed to get video mode from primary monitor!"
                    );
                    (0, 0)
                }
            } else {
                error!("Tomato:GLFW", "Failed to find primary monitor!");
                (0, 0)
            }
        });

        let window: Option<(glfw::Window, Receiver<(f64, WindowEvent)>)> =
            glfw.create_window(width, height, config.title, WindowMode::Windowed);
        if let Some((mut window_ptr, events)) = window {
            // Center the window
            window_ptr.set_pos(x, y);

            // Enable all events
            window_ptr.set_all_polling(true);

            // Make current
            window_ptr.make_current();
            gl::load_with(|symbol| window_ptr.get_proc_address(symbol) as *const _);

            return Window {
                window_ptr,
                events,
                width: config.width,
                height: config.height,
                scale_x: 1.0,
                scale_y: 1.0,
                x,
                y,
                input: Input::new(),
                size_changed: true,
                pos_changed: true,
            };
        } else {
            critical!("Tomato:GLFW", "Failed to create window!");
        }
    }

    pub fn update(&mut self) {
        // Reset flags
        self.size_changed = false;
        self.pos_changed = false;

        // Update Input
        self.input.update();

        // Update
        self.window_ptr.glfw.poll_events();

        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                WindowEvent::Pos(x, y) => {
                    self.x = x;
                    self.y = y;
                    self.pos_changed = true;
                }
                WindowEvent::FramebufferSize(width, height) => {
                    self.scale_x *= width as f32 / self.width as f32;
                    self.scale_y *= height as f32 / self.height as f32;
                    self.width = width as u32;
                    self.height = height as u32;
                    self.size_changed = true;
                }
                WindowEvent::MouseButton(button, action, _) => match action {
                    glfw::Action::Release => {
                        self.input.mouse.buttons[button as usize] = Action::Released
                    }
                    glfw::Action::Press => {
                        self.input.mouse.buttons[button as usize] = Action::Pressed
                    }
                    _ => {}
                },
                WindowEvent::CursorPos(mx, my) => {
                    self.input.mouse.x = mx as i32;
                    self.input.mouse.y = my as i32;
                }
                WindowEvent::Scroll(sx, sy) => {
                    self.input.mouse.scroll_x = sx;
                    self.input.mouse.scroll_y = sy;
                }
                WindowEvent::Key(key, _, action, _) => match action {
                    glfw::Action::Release => {
                        self.input.keyboard.keys[key as usize] = Action::Released
                    }
                    glfw::Action::Press => self.input.keyboard.keys[key as usize] = Action::Pressed,
                    _ => {}
                },
                WindowEvent::Char(_) => {}
                WindowEvent::CharModifiers(_, _) => {}
                WindowEvent::FileDrop(_) => {}
                _ => {}
            }
        }
    }

    pub fn swap_buffers(&mut self) {
        self.window_ptr.swap_buffers();
    }

    // Getters
    pub fn get_width(&self) -> u32 {
        return self.width;
    }

    pub fn get_height(&self) -> u32 {
        return self.height;
    }

    pub fn get_scale_x(&self) -> f32 {
        return self.scale_x;
    }

    pub fn get_scale_y(&self) -> f32 {
        return self.scale_y;
    }

    pub fn get_x(&self) -> i32 {
        return self.x;
    }

    pub fn get_y(&self) -> i32 {
        return self.y;
    }

    pub fn should_close(&self) -> bool {
        return self.window_ptr.should_close();
    }

    pub fn input(&self) -> &Input {
        return &self.input;
    }

    pub fn set_vsync(&mut self, enable: bool) {
        self.window_ptr.glfw.set_swap_interval(
            enable
                .then(|| SwapInterval::Sync(1))
                .unwrap_or(SwapInterval::None),
        );
    }

    // Setters
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.window_ptr.set_size(width as i32, height as i32);
    }

    pub fn set_width(&mut self, width: u32) {
        self.set_size(width, self.height);
    }

    pub fn set_height(&mut self, height: u32) {
        self.set_size(self.width, height);
    }

    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
        self.window_ptr.set_pos(x, y);
    }

    pub fn set_x(&mut self, x: i32) {
        self.set_pos(x, self.y);
    }

    pub fn set_y(&mut self, y: i32) {
        self.set_pos(self.x, y);
    }

    pub fn close(&mut self) {
        self.window_ptr.set_should_close(true);
    }

    pub fn show(&mut self) {
        self.window_ptr.show();
    }

    pub fn hide(&mut self) {
        self.window_ptr.hide();
    }

    pub fn get_time(&self) -> f64 {
        return self.window_ptr.glfw.get_time();
    }

    pub fn size_changed(&self) -> bool {
        return self.size_changed;
    }
}
