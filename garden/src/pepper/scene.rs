use crate::{mem::MutRef, ui::{get_system, IUIElement, SceneComponent}, potato::Assume};

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                              Scene                                             //
////////////////////////////////////////////////////////////////////////////////////////////////////

static mut SCENE: Option<String> = None;

pub fn get_scene() -> Option<String> {
    unsafe {
        return SCENE.clone();
    }
}

pub fn set_scene(scene: Option<String>) {
    unsafe {
        SCENE = scene;
    }
}

pub fn prepare_elements() {
    for element in get_system().iter_entity_mut() {
        MutRef::from(element).prepare();
    }
}

pub fn update_scene(delta: f64) {
    if get_scene().is_none() {
        return;
    }

    for element in get_system().iter_entity_mut() {
        if element.get_component::<SceneComponent>().assume("Pepper", format!("Scene Component not defined for '{}'!", element.id()).as_str()).0 == get_scene() {
            MutRef::from(element).update(delta);
        }
    }
}