use std::slice::{Iter, IterMut};
use std::{any::Any};
use std::collections::HashMap;

use crate::lettuce::Entity;
use crate::mem::{MutRef, Ref};

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                             System                                             //
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct System<Id: PartialEq> {
    entities: Vec<Entity<Id>>,
    on_entity_created: Box<dyn Fn(&mut Entity<Id>)>,
    pub(crate) on_component_added: Box<dyn Fn(MutRef<Entity<Id>>, &mut Box<dyn Any>)>,
    attributes: HashMap<String, Box<dyn Any>>
}

impl<Id: PartialEq> System<Id> {

    pub fn new() -> System<Id> {
        return System::new_custom(|_| {}, |_, _| {});
    }

    pub fn new_custom<F: Fn(&mut Entity<Id>) + 'static, G: Fn(MutRef<Entity<Id>>, &mut Box<dyn Any>) + 'static>(on_entity_created: F, on_component_added: G) -> System<Id> {
        return System {
            entities: Vec::new(),
            on_entity_created: Box::new(on_entity_created),
            on_component_added: Box::new(on_component_added),
            attributes: HashMap::new()
        }
    }

    pub fn add_attribute<T: Any + 'static>(&mut self, name: &str, value: T) {
        self.attributes.insert(name.to_string(), Box::new(value));
    }

    pub fn get_attribute<T: Any + 'static>(&self, name: &str) -> Option<Ref<T>> {
        return Some(Ref::from(self.attributes.get(&name.to_string())?.downcast_ref::<T>()?));
    }

    pub fn get_attribute_mut<T: Any + 'static>(&mut self, name: &str) -> Option<MutRef<T>> {
        return Some(MutRef::from(self.attributes.get_mut(&name.to_string())?.downcast_mut::<T>()?));
    }

    pub fn get_entity(&mut self, id: Id) -> Ref<Entity<Id>> {
        return if let Some(entity) = self.entities.iter().find(|e| e.id() == &id) {
            Ref::from(entity)
        } else {
            let mut entity = Entity::new(MutRef::new(self as *mut System<Id>), id);
            (self.on_entity_created)(&mut entity);
            self.entities.push(entity);
            Ref::from(self.entities.last().unwrap())
        }
    }

    pub fn get_entity_mut(&mut self, id: Id) -> MutRef<Entity<Id>> {
        return if let Some(entity) = self.entities.iter_mut().find(|e| e.id() == &id) {
            MutRef::from(entity)
        } else {
            let mut entity = Entity::new(MutRef::new(self as *mut System<Id>), id);
            (self.on_entity_created)(&mut entity);
            self.entities.push(entity);
            MutRef::from(self.entities.last_mut().unwrap())
        }
    }

    pub fn query_entity<F: Fn(&Entity<Id>) -> bool>(&self, predicate: F) -> Option<Ref<Entity<Id>>> {
        return Some(Ref::from(self.entities.iter().find(|e| predicate(*e))?));
    }

    pub fn query_entities<F: Fn(&Entity<Id>) -> bool>(&self, predicate: F) -> Vec<Ref<Entity<Id>>> {
        return self.entities.iter().filter_map(|entity| predicate(entity).then(|| Ref::new(entity))).collect();
    }

    pub fn query_entity_mut<F: Fn(&Entity<Id>) -> bool>(&mut self, predicate: F) -> Option<MutRef<Entity<Id>>> {
        return Some(MutRef::from(self.entities.iter_mut().find(|e| predicate(*e))?));
    }

    pub fn query_entities_mut<F: Fn(&Entity<Id>) -> bool>(&mut self, predicate: F) -> Vec<MutRef<Entity<Id>>> {
        return self.entities.iter_mut().filter_map(|entity| predicate(entity).then(|| MutRef::new(entity))).collect();
    }

    pub fn query_component<Component: Any + 'static, F: Fn(&Entity<Id>) -> bool>(&self, predicate: F) -> Option<Ref<Component>> {
        return self.entities.iter().find_map(|entity| predicate(entity).then(|| entity.get_component::<Component>())?);
    }

    pub fn query_components<Component: Any + 'static, F: Fn(&Entity<Id>) -> bool>(&self, predicate: F) -> Vec<Ref<Component>> {
        return self.entities.iter().filter_map(|entity| predicate(entity).then(|| entity.get_component::<Component>())?).collect();
    }

    pub fn query_component_mut<Component: Any + 'static, F: Fn(&Entity<Id>) -> bool>(&mut self, predicate: F) -> Option<MutRef<Component>> {
        return self.entities.iter_mut().find_map(|entity| predicate(entity).then(|| entity.get_component_mut::<Component>())?);
    }

    pub fn query_components_mut<Component: Any + 'static, F: Fn(&Entity<Id>) -> bool>(&mut self, predicate: F) -> Vec<MutRef<Component>> {
        return self.entities.iter_mut().filter_map(|entity| predicate(entity).then(|| entity.get_component_mut::<Component>())?).collect();
    }

    pub fn query_entity_with<Component: Any + 'static, F: Fn(&Entity<Id>) -> bool>(&self, predicate: F) -> Option<(Ref<Entity<Id>>, Ref<Component>)> {
        return self.entities.iter().find_map(|entity| {
            predicate(entity).then(|| {
                if let Some(component) = entity.get_component::<Component>() {
                    Some((Ref::from(entity), component))
                } else {
                    None
                }
            })?
        });
    }

    pub fn query_entities_with<Component: Any + 'static, F: Fn(&Entity<Id>) -> bool>(&self, predicate: F) -> Vec<(Ref<Entity<Id>>, Ref<Component>)> {
        return self.entities.iter().filter_map(|entity| {
            predicate(entity).then(|| {
                if let Some(component) = entity.get_component::<Component>() {
                    Some((Ref::from(entity), component))
                } else {
                    None
                }
            })?
        }).collect();
    }

    pub fn query_entity_with_mut<Component: Any + 'static, F: Fn(&Entity<Id>) -> bool>(&mut self, predicate: F) -> Option<(MutRef<Entity<Id>>, MutRef<Component>)> {
        return self.entities.iter_mut().find_map(|entity| {
            predicate(entity).then(|| {
                if let Some(component) = entity.get_component_mut::<Component>() {
                    Some((MutRef::from(entity), component))
                } else {
                    None
                }
            })?
        });
    }

    pub fn query_entities_with_mut<Component: Any + 'static, F: Fn(&Entity<Id>) -> bool>(&mut self, predicate: F) -> Vec<(MutRef<Entity<Id>>, MutRef<Component>)> {
        return self.entities.iter_mut().filter_map(|entity| {
            predicate(entity).then(|| {
                if let Some(component) = entity.get_component_mut::<Component>() {
                    Some((MutRef::from(entity), component))
                } else {
                    None
                }
            })?
        }).collect();
    }

    pub fn iter_entity(&self) -> Iter<'_, Entity<Id>> {
        self.entities.iter()
    }

    pub fn iter_entity_mut(&mut self) -> IterMut<'_, Entity<Id>> {
        self.entities.iter_mut()
    }

}