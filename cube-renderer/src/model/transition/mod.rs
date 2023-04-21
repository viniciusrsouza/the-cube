use crate::console;

pub mod easing;

pub struct Transition<T> {
    start: T,
    end: T,
    duration: f32,
    elapsed: f32,
    function: fn(f32) -> f32,
}

impl<T> Transition<T> {
    pub fn new(start: T, end: T, duration: f32, function: fn(f32) -> f32) -> Self {
        Self {
            start,
            end,
            duration,
            elapsed: 0.0,
            function,
        }
    }

    fn get_time(&self) -> f32 {
        let t = self.elapsed / self.duration;
        if t > 1.0 {
            1.0
        } else {
            t
        }
    }

    pub(crate) fn is_finished(&self) -> bool {
        self.elapsed >= self.duration
    }
}

impl Transition<glm::Vec3> {
    pub fn update(&mut self, dt: f32) -> glm::Vec3 {
        self.elapsed += dt;
        let time = self.get_time();
        console::log!("time: {}", time);
        let c = (self.function)(time);
        self.start + c * (self.end - self.start)
    }
}

pub trait TransitionFn {
    fn apply(&mut self, dt: f32) -> f32;
}
