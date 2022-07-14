use std::any::TypeId;

use crate::{mem::MutRef, ui::UIElement, trait_component};

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                           UIComponent                                          //
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct UIComponentBase {
    pub parent: Option<MutRef<UIElement>>,
    pub z_index: i32
}

impl Default for UIComponentBase {
    fn default() -> Self {
        Self { parent: None, z_index: 0}
    }
}

pub trait UIComponent {

    fn update(&mut self, delta: f64);

    fn get_parent(&mut self) -> Option<MutRef<UIElement>>;
    fn set_parent(&mut self, parent: Option<MutRef<UIElement>>);

    fn get_z_index(&mut self) -> i32;
    fn set_z_index(&mut self, z_index: i32);

    fn calculate_z_index(&mut self) -> i32 {
        if let Some(parent) = self.get_parent() {
            return parent.get_component::<ZIndexComponent>().map_or(0, |comp| comp.0) * 100 + self.get_z_index();
        }
        return self.get_z_index();
    }

    fn component_id(&self) -> TypeId;

}
trait_component!(UIComponent);

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                       Default Components                                       //
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone)]
pub struct ZIndexComponent(pub i32);

#[derive(Clone)]
pub struct ParentComponent(pub Option<String>);

#[derive(Clone)]
pub struct SceneComponent(pub Option<String>);