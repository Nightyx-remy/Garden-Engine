use crate::{garlic::Color, carrot::Renderer2D, onion::App};

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                            Renderer                                            //
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Renderer {
    pub clear_color: Color,
    pub r2d: Renderer2D,
}

impl Renderer {

    pub fn new() -> Renderer {
        return Renderer {
            clear_color: Color::white(),
            r2d: Renderer2D::new(),
        }
    }

    pub fn init(&mut self) {
        unsafe {
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::BLEND);
        }

        self.r2d.init();
        self.resize(App::get().window().get_width(), App::get().window().get_height());
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);

            self.r2d.resize(width, height);
        }
    }

    pub fn clear(&self) {
        unsafe {
            gl::ClearColor(self.clear_color.red, self.clear_color.green, self.clear_color.blue, self.clear_color.alpha);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn render(&mut self) {
        self.r2d.render();
    }

}