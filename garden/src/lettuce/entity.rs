use std::any::Any;

use crate::{mem::{MutRef, Ref}, lettuce::System};

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                             Entity                                             //
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Entity<Id: PartialEq> {
    system: MutRef<System<Id>>,
    id: Id,
    components: Vec<Box<dyn Any>>
}

impl<Id: PartialEq> Entity<Id> {

    pub(crate) fn new(system: MutRef<System<Id>>, id: Id) -> Entity<Id> {
        return Entity {
            system,
            id,
            components: Vec::new()
        }
    }

    pub fn add_component<Component: Any + 'static>(&mut self, component: Component) {
        let self_ref = MutRef::new(self as *mut Entity<Id>);
        let mut component: Box<dyn Any> = Box::new(component);
        (self.system.on_component_added)(self_ref, &mut component);
        self.components.push(component);
    }

    pub fn get_component<Component: Any + 'static>(&self) -> Option<Ref<Component>> {
        for component in self.components.iter() {
            if component.is::<Component>() {
                return Some(Ref::from(component.downcast_ref::<Component>()?));
            }
        }
        return None;
    }

    pub fn get_component_mut<Component: Any + 'static>(&mut self) -> Option<MutRef<Component>> {
        for component in self.components.iter_mut() {
            if component.is::<Component>() {
                return Some(MutRef::from(component.downcast_mut::<Component>()?));
            }
        }
        return None;
    }

    pub fn get_components<Component: Any + 'static>(&self) -> Vec<Ref<Component>> {
        let mut comps = Vec::new();
        for component in self.components.iter() {
            if component.is::<Component>() {
                if let Some(comp) = component.downcast_ref::<Component>() {
                    comps.push(Ref::from(comp));
                }
            }
        }
        return comps;
    }

    pub fn get_components_mut<Component: Any + 'static>(&mut self) -> Vec<MutRef<Component>> {
        let mut comps = Vec::new();
        for component in self.components.iter_mut() {
            if component.is::<Component>() {
                if let Some(comp) = component.downcast_mut::<Component>() {
                    comps.push(MutRef::from(comp));
                }
            }
        }
        return comps;
    }

    pub fn id(&self) -> &Id {
        return &self.id;
    }

}