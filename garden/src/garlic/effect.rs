////////////////////////////////////////////////////////////////////////////////////////////////////
//                                          Effect Mode                                           //
////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::onion::App;

pub enum EffectMode {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
}

impl EffectMode {

    pub fn calculate(&mut self, duration: f64, delta: f64) -> f64 {
        let mut linear = delta / duration;
        match self {
            EffectMode::Linear => linear,
            EffectMode::EaseIn => linear*linear,
            EffectMode::EaseOut => -linear * (linear - 2.0),
            EffectMode::EaseInOut => {
                return if linear < 0.5 {
                    2.0 * linear * linear
                } else {
                    linear -= 1.0;
                    -2.0 * (linear - 1.0) * (linear - 1.0)
                }
            },
        }
    }

}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                          Interpolate                                           //
////////////////////////////////////////////////////////////////////////////////////////////////////

pub trait Interpolate<O, T> {

    fn interpolate(start: T, target: T, value: f64) -> O;

}

impl Interpolate<f32, f32> for f32 {
    fn interpolate(start: f32, target: f32, value: f64) -> f32 {
        return (target - start) * value as f32 + start;
    }
}

impl Interpolate<f64, f64> for f64 {

    fn interpolate(start: f64, target: f64, value: f64) -> f64 {
        return (target - start) * value + start;
    }

}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                          Effect State                                          //
////////////////////////////////////////////////////////////////////////////////////////////////////


#[derive(Clone, Copy)]
pub enum EffectState {
    Started(f64),
    Stopped
}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                             Effect                                             //
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Effect<T: Interpolate<O, T>, O = T> {
    start: T,
    target: Option<T>,
    current: O,
    state: EffectState,
    duration: f64,
    mode: EffectMode,
    repeat: bool
}

impl<T: Interpolate<O, T> + Clone, O: Clone> Effect<T, O> {

    pub fn new(value: T) -> Effect<T, O> {
        let current: O = T::interpolate(value.clone(), value.clone(), 0.0);
        return Effect {
            start: value,
            target: None,
            current: current,
            state: EffectState::Stopped,
            duration: 1.0,
            mode: EffectMode::Linear,
            repeat: true,
        }
    }

    pub fn update(&mut self) {
        let now = App::get().window().get_time(); 
        if let (EffectState::Started(start_time), Some(target)) = (self.state, &mut self.target) {
            if now >= start_time + self.duration {
                if self.repeat {
                    self.state = EffectState::Started(start_time + self.duration);
                    std::mem::swap(&mut self.start, target);            
                    self.update();            
                } else {
                    self.state = EffectState::Stopped;
                    self.current = T::interpolate(self.start.clone(), target.clone(), 1.0);
                }
            } else {
                let value = self.mode.calculate(self.duration, now - start_time);
                self.current = T::interpolate(self.start.clone(), target.clone(), value);
            }
        } else if let EffectState::Started(_) = self.state {
            self.state = EffectState::Stopped;
        }
    }

    pub fn set_fixed(&mut self, value: T) -> &mut Effect<T, O> {
        let current = T::interpolate(value.clone(), value.clone(), 0.0);
        self.start = value.clone();
        self.target = None;
        self.current = current;
        self.state = EffectState::Stopped;
        return self;
    }

    pub fn set_animation(&mut self, start: T, target: T, duration: f64, mode: EffectMode, repeat: bool) -> &mut Effect<T, O> {
        let current = T::interpolate(start.clone(), target.clone(), 0.0);
        self.start = start.clone();
        self.target = Some(target);
        self.current = current;
        self.state = EffectState::Stopped;
        self.duration = duration.max(0.001); // Minimum = 1ms
        self.mode = mode;
        self.repeat = repeat;
        return self;
    }

    pub fn start(&mut self) -> &mut Effect<T, O> {
        let now = App::get().window().get_time(); 
        self.state = EffectState::Started(now);
        return self;
    }

    pub fn stop(&mut self) -> &mut Effect<T, O> {
        self.state = EffectState::Stopped;
        return self;
    }

    pub fn current(&self) -> O {
        return self.current.clone();
    }

}