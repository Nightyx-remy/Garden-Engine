use std::any::{Any, TypeId};

use crate::{mem::MutRef, warn};

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                              State                                             //
////////////////////////////////////////////////////////////////////////////////////////////////////

pub trait IState: Any {

    fn init(&mut self);
    fn open(&mut self);
    fn update(&mut self, delta: f64);
    fn close(&mut self);
    fn dispose(&mut self);

}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                          State Manager                                         //
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct StateManager {
    states: Vec<Box<dyn IState>>,
    current: Option<MutRef<Box<dyn IState>>>,
    initialized: bool,
    disposed: bool
}

impl StateManager {

    pub fn new() -> StateManager {
        return StateManager {
            states: vec![],
            current: None,
            initialized: false,
            disposed: false
        }
    }

    pub fn register<State: IState + 'static>(&mut self, mut state: State) {
        if self.initialized && !self.disposed {
            state.init();
        }
        self.states.push(Box::new(state));
    }

    pub fn get_state<State: IState + 'static>(&mut self) -> Option<MutRef<State>> {
        for state in self.states.iter_mut() {
            if (**state).type_id() == TypeId::of::<State>() {
                unsafe {
                    return Some(MutRef::from(&mut **(state as *mut dyn Any as *mut Box<State>)))
                }
            }
        }
        return None;
    }

    fn get_state_ref<State: IState + 'static>(&mut self) -> Option<MutRef<Box<dyn IState>>> {
        for state in self.states.iter_mut() {
            if (**state).type_id() == TypeId::of::<State>() {
                return Some(MutRef::from(state));
            }
        }
        warn!("Onion", "State '{}' not found!", std::any::type_name::<State>());
        return None;
    }

    pub fn init(&mut self) {
        if !self.initialized {
            self.states.iter_mut().for_each(|state| state.init());
            if let Some(current) = &mut self.current {
                current.open();
            }
            self.initialized = true;
        } else {
            warn!("Onion", "Failed to initialize StateManager, already initialized!");
        }
    }

    pub fn open<State: IState + 'static>(&mut self) {
        self.close();
        let state = self.get_state_ref::<State>();
        if let Some(mut state) = state {
            if self.initialized && !self.disposed {
                state.open();
            }
            self.current = Some(state);
        }
    }

    pub fn update(&mut self, delta: f64) {
        if self.initialized && !self.disposed {
            if let Some(current) = &mut self.current {
                current.update(delta);
            }
        }
    }

    pub fn close(&mut self) {
        if let Some(mut current) = self.current.take() {
            if self.initialized && !self.disposed {
                current.close();
            }
        }
    }

    pub fn dispose(&mut self) {
        if self.initialized && !self.disposed {
            self.close();
            self.states.iter_mut().for_each(|state| state.dispose());
            self.disposed = true;
        } else {
            warn!("Onion", "Failed to dispose StateManager, already disposed!");
        }
    }

}