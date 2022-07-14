use crate::{potato::{LogLevel, Logger}, window::{WindowConfig, Window}, onion::{StateManager, Renderer}, mem::MutRef, critical, ui::{get_system, ScreenProperty, prepare_elements, update_scene}, debug::get_errors};

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                           Update Cap                                           //
////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum UpdateCap {
    Cap(u32),
    Vsync,
    Unlimited
}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                        App Configuration                                       //
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct AppConfiguration<'a> {
    pub update_cap: UpdateCap,
    pub log_level: LogLevel,
    pub window_config: WindowConfig<'a>
}

impl<'a> Default for AppConfiguration<'a> {

    fn default() -> Self {
        return AppConfiguration {
            update_cap: UpdateCap::Vsync,
            log_level: LogLevel::Info,
            window_config: WindowConfig::default()
        }
    }

}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                               App                                              //
////////////////////////////////////////////////////////////////////////////////////////////////////

static mut APP: Option<App> = None;

pub struct App {
    pub window: Option<Window>,
    pub state_manager: StateManager,
    pub update_cap: UpdateCap,
    pub renderer: Renderer
}

impl App {

    pub fn get() -> MutRef<App> {
        unsafe {
            return if let Some(app) = &mut APP {
                MutRef::from(app)
            } else {
                APP = Some(Self::new());
                MutRef::from(APP.as_mut().unwrap())
            }
        }
    }

    fn new() -> App {
        return App {
            window: None,
            state_manager: StateManager::new(),
            update_cap: UpdateCap::Vsync,
            renderer: Renderer::new(),
        }
    }

    pub fn setup(&mut self, config: AppConfiguration) -> &mut Self {
        self.window = Some(Window::new(config.window_config));
        self.update_cap = config.update_cap;

        Logger::get().set_level(config.log_level);

        self.window.as_mut().unwrap().set_vsync(match self.update_cap {
            UpdateCap::Cap(_) | UpdateCap::Unlimited => false,
            UpdateCap::Vsync => true,
        });

        self.renderer.init();

        self.state_manager.init();

        self.window.as_mut().unwrap().show();
        self
    }

    pub fn start(&mut self) {
        if self.window.is_none() {
            critical!("Onion", "The application hasn't been initialized, call setup() first!");
        }

        let mut last_time = self.window.as_ref().unwrap().get_time();
        while !self.window.as_ref().unwrap().should_close() {
            let now = self.window.as_ref().unwrap().get_time();
            let delta = now - last_time;

            if match self.update_cap {
                UpdateCap::Cap(cap) if 1.0 / (cap as f64) <= delta => true,
                UpdateCap::Vsync | UpdateCap::Unlimited => true,
                _ => false,
            } {
                self.window.as_mut().unwrap().update();

                if self.window.as_mut().unwrap().size_changed() {
                    self.renderer.resize(self.window.as_mut().unwrap().get_width(), self.window.as_mut().unwrap().get_height());

                    if let Some(mut screen) = get_system().get_attribute_mut::<ScreenProperty>("screen") {
                        screen.width = self.window().get_width();
                        screen.height = self.window().get_height();
                        screen.scale_x = self.window().get_scale_x();
                        screen.scale_y = self.window().get_scale_y();
                    } else {
                        get_system().add_attribute("screen", ScreenProperty {
                            width: self.window().get_width(),
                            height: self.window().get_height(),
                            scale_x: self.window().get_scale_x(),
                            scale_y: self.window().get_scale_y(),
                        })
                    }
                }

                // Retrieve GL Errors
                get_errors();

                prepare_elements();

                self.renderer.clear();
                self.state_manager.update(delta);

                update_scene(delta);

                self.renderer.render();

                last_time = now;
                self.window.as_mut().unwrap().swap_buffers();
            }
        }

        self.state_manager.dispose();
    }

    pub fn window(&mut self) -> MutRef<Window> {
        if self.window.is_none() {
            critical!("Onion", "The application hasn't been initialized, call setup() first!");
        }

        return MutRef::new(self.window.as_mut().unwrap());
    }

    pub fn state_manager(&mut self) -> MutRef<StateManager> {
        return MutRef::from(&mut self.state_manager);
    }

    pub fn renderer(&mut self) -> MutRef<Renderer> {
        return MutRef::from(&mut self.renderer);
    }

}