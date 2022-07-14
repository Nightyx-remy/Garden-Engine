use std::any::TypeId;

use crate::{ui::{UIComponent, ZIndexComponent, ParentComponent, SceneComponent, Constraints}, mem::MutRef, lettuce::Entity, potato::Assume};

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                           Element Ref                                          //
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Eq, PartialEq)]
pub enum ElementRef {
    This,
    Parent,
    Screen,
    Other(String)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                           UI Element                                           //
////////////////////////////////////////////////////////////////////////////////////////////////////

pub trait IUIElement {

    fn add_ui_component<Component: UIComponent + 'static>(&mut self, component: Component);
    fn get_ui_component_mut<Component: UIComponent + 'static>(&mut self) -> Option<MutRef<Component>>;

    fn prepare(&mut self);
    fn update(&mut self, delta: f64);

    // Default Components
    fn get_constraints(&mut self) -> MutRef<Constraints>;
    fn get_z_index(&mut self) -> MutRef<ZIndexComponent>;
    fn get_parent(&mut self) -> MutRef<ParentComponent>;
    fn get_scene(&mut self) -> MutRef<SceneComponent>;

}

pub type UIElement = Entity<String>;

impl IUIElement for UIElement {

    fn add_ui_component<Component: UIComponent + 'static>(&mut self, component: Component) {
        self.add_component(Box::new(component) as Box<dyn UIComponent>);
    }

    fn get_ui_component_mut<Component: UIComponent + 'static>(&mut self) -> Option<MutRef<Component>> {
        for mut component in self.get_components_mut::<Box<dyn UIComponent>>() {
            if component.component_id() == TypeId::of::<Component>() {
                return Some(MutRef::from(component.downcast_mut::<Component>()));
            } 
        }
        None
    }

    fn prepare(&mut self) {
        self.get_constraints().prepare();
    }

    fn update(&mut self, delta: f64) {
        for mut component in self.get_component_mut::<Box<dyn UIComponent>>() {
            component.update(delta);
        }
    }

    fn get_constraints(&mut self) -> MutRef<Constraints> {
        return self.get_component_mut::<Constraints>().assume("Pepper", format!("Constraints Component not defined for '{}'!", self.id()).as_str());
    }

    fn get_z_index(&mut self) -> MutRef<ZIndexComponent> {
        return self.get_component_mut::<ZIndexComponent>().assume("Pepper", format!("ZIndex Component not defined for '{}'!", self.id()).as_str());
    }

    fn get_parent(&mut self) -> MutRef<ParentComponent> {
        return self.get_component_mut::<ParentComponent>().assume("Pepper", format!("Parent Component not defined for '{}'!", self.id()).as_str());
    }

    fn get_scene(&mut self) -> MutRef<SceneComponent> {
        return self.get_component_mut::<SceneComponent>().assume("Pepper", format!("Scene Component not defined for '{}'!", self.id()).as_str());
    }

}