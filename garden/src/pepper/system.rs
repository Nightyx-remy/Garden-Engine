use std::any::Any;

use crate::{lettuce::{System, Entity}, ui::{Constraints, ZIndexComponent, ParentComponent, SceneComponent, UIComponent}, mem::MutRef};

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                            UI System                                           //
////////////////////////////////////////////////////////////////////////////////////////////////////

pub type UISystem = System<String>;

static mut SYSTEM: Option<UISystem> = None;

pub fn get_system() -> MutRef<UISystem> {
    unsafe {
        return if let Some(system) = &mut SYSTEM {
            MutRef::from(system)
        } else {
            SYSTEM = Some({
                let mut system = System::new_custom(|entity: &mut Entity<String>| {
                    entity.add_component({
                        let mut constraints = Constraints::default();
                        constraints.apply_to(entity.id().clone());
                        constraints
                    });
                    entity.add_component(ZIndexComponent(0));
                    entity.add_component(ParentComponent(None));
                    entity.add_component(SceneComponent(None));
                }, |entity: MutRef<Entity<String>>, component: &mut Box<dyn Any>| {
                    if component.is::<Box<dyn UIComponent>>() {
                        if let Some(comp) = component.downcast_mut::<Box<dyn UIComponent>>() {
                            comp.set_parent(Some(entity));
                        }
                    }
                });
                system.add_attribute("screen", ScreenProperty {
                    width: 800,
                    height: 600,
                    scale_x: 1.0,
                    scale_y: 1.0
                });
                system
            });
            MutRef::from(SYSTEM.as_mut().unwrap())
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                         Screen Property                                        //
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct ScreenProperty {
    pub width: u32,
    pub height: u32,
    pub scale_x: f32,
    pub scale_y: f32,
}