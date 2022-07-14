use super::Interpolate;

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                             Color                                              //
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32
}

impl Color {

    // Predefined colors
    pub fn white() -> Color {
        return Color::from_rgb(1.0, 1.0, 1.0);
    }

    pub fn light_gray() -> Color {
        return Color::from_rgb(0.75, 0.75, 0.75);
    }

    pub fn gray() -> Color {
        return Color::from_rgb(0.5, 0.5, 0.5);
    }

    pub fn dark_gray() -> Color {
        return Color::from_rgb(0.25, 0.25, 0.25);
    }

    pub fn black() -> Color {
        return Color::from_rgb(0.0, 0.0, 0.0);
    }

    pub fn red() -> Color {
        return Color::from_rgb(1.0, 0.0, 0.0);
    }

    pub fn green() -> Color {
        return Color::from_rgb(0.0, 1.0, 0.0);
    }

    pub fn blue() -> Color {
        return Color::from_rgb(0.0, 0.0, 1.0);
    }

    pub fn yellow() -> Color {
        return Color::from_rgb(1.0, 1.0, 0.0);
    }

    pub fn magenta() -> Color {
        return Color::from_rgb(1.0, 0.0, 1.0);
    }

    pub fn teal() -> Color {
        return Color::from_rgb(0.0, 1.0, 1.0);
    }

    pub fn transparent() -> Color {
        return Color::from_rgba(0.0, 0.0, 0.0, 0.0);
    }

    // Constructors
    pub fn from_rgba(red: f32, green: f32, blue: f32, alpha: f32) -> Color {
        return Color {
            red,
            green,
            blue,
            alpha
        }
    }

    pub fn from_rgb(red: f32, green: f32, blue: f32) -> Color {
        return Color::from_rgba(red, green, blue, 1.0);
    }

    pub fn from_rgba255(red: i32, green: i32, blue: i32, alpha: i32) -> Color {
        return Color::from_rgba(red as f32 / 255.0, green as f32 / 255.0, blue as f32 / 255.0, alpha as f32 / 255.0);
    }

    pub fn from_rgb255(red: i32, green: i32, blue: i32) -> Color {
        return Color::from_rgb(red as f32 / 255.0, green as f32 / 255.0, blue as f32 / 255.0);
    }

    pub fn from_rgba_hex(rgba: i64) -> Color {
        return Color::from_rgba255(
            ((rgba >> 24) & 0xFF) as i32,
            ((rgba >> 16) & 0xFF) as i32,
            ((rgba >> 8) & 0xFF) as i32,
            (rgba & 0xFF) as i32
        );
    }

    pub fn from_rgb_hex(rgba: i64) -> Color {
        return Color::from_rgb255(
            ((rgba >> 16) & 0xFF) as i32,
            ((rgba >> 8) & 0xFF) as i32,
            (rgba & 0xFF) as i32
        );
    }

    // Getters
    pub fn get_red255(&self) -> i32 {
        return (self.red * 255.0) as i32;
    }

    pub fn get_green255(&self) -> i32 {
        return (self.green * 255.0) as i32;
    }

    pub fn get_blue255(&self) -> i32 {
        return (self.blue * 255.0) as i32;
    }

    pub fn get_alpha255(&self) -> i32 {
        return (self.alpha * 255.0) as i32;
    }

    pub fn get_rgba_hex(&self) -> i64 {
        return (self.get_red255() << 24) as i64 |
            (self.get_green255() << 16) as i64 |
            (self.get_blue255() << 8) as i64 |
            self.get_alpha255() as i64;
    }

    pub fn get_rgb_hex(&self) -> i64 {
        return (self.get_red255() << 16) as i64 |
            (self.get_green255() << 8) as i64 |
            self.get_blue255() as i64;
    }

    // Setters
    pub fn set_red255(&mut self, red: i32) {
        self.red = red as f32 / 255.0;
    }

    pub fn set_green255(&mut self, green: i32) {
        self.green = green as f32 / 255.0;
    }

    pub fn set_blue255(&mut self, blue: i32) {
        self.blue = blue as f32 / 255.0;
    }

    pub fn set_alpha255(&mut self, alpha: i32) {
        self.alpha = alpha as f32 / 255.0;
    }

    pub fn set_rgba_hex(&mut self, rgba: i64) {
        self.set_red255(((rgba << 24) & 0xFF) as i32);
        self.set_green255(((rgba << 16) & 0xFF) as i32);
        self.set_blue255(((rgba << 8) & 0xFF) as i32);
        self.set_alpha255((rgba & 0xFF) as i32);
    }

    pub fn set_rgb_hex(&mut self, rgba: i64) {
        self.set_red255(((rgba << 16) & 0xFF) as i32);
        self.set_green255(((rgba << 8) & 0xFF) as i32);
        self.set_blue255((rgba & 0xFF) as i32);
    }

}

impl Interpolate<Color, Color> for Color {

    fn interpolate(start: Color, target: Color, value: f64) -> Color {
        return Color::from_rgba(
            f32::interpolate(start.red, target.red, value),
            f32::interpolate(start.green, target.green, value),
            f32::interpolate(start.blue, target.blue, value),
            f32::interpolate(start.alpha, target.alpha, value)
        );
    }

}