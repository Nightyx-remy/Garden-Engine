use std::any::Any;

use crate::{ui::{UIComponentBase, UIComponent, UIElement, IUIElement}, garlic::{Color, Effect}, onion::App, mem::MutRef};

pub struct BackgroundComponent {
    base: UIComponentBase,
    pub color: Effect<Color>
}

impl BackgroundComponent {

    pub fn new(color: Color) -> BackgroundComponent {
        return BackgroundComponent { 
            base: UIComponentBase::default(), 
            color: Effect::new(color)
        }
    }

}

impl UIComponent for BackgroundComponent {
    fn update(&mut self, _delta: f64) {
        if self.get_parent().is_none() {
            return;
        }

        let mut constraints = self.get_parent().unwrap().get_constraints();
        self.color.update();
        App::get().renderer().r2d.fill_rect(constraints.get_x(), constraints.get_y(), constraints.get_width(), constraints.get_height(), self.color.current());
    }

    fn get_parent(&mut self) -> Option<MutRef<UIElement>> {
        return self.base.parent;
    }

    fn set_parent(&mut self, parent: Option<MutRef<UIElement>>) {
        self.base.parent = parent;
    }

    fn get_z_index(&mut self) -> i32 {
        return self.base.z_index;
    }

    fn set_z_index(&mut self, z_index: i32) {
        self.base.z_index = z_index;
    }

    fn component_id(&self) -> std::any::TypeId {
        return self.type_id();
    }
}