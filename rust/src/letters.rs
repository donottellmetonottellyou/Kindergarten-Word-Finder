use godot::{
    engine::{ISprite2D, Sprite2D},
    obj::WithBaseField,
    prelude::*,
};
use rand::prelude::*;

use std::time::{Duration, Instant};

#[derive(Default, Clone, Copy, GodotConvert, Var, Export)]
#[repr(i32)]
#[godot(via = GString)]
enum Letter {
    #[default]
    A = 0,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct ExtLetterSprite {
    #[export]
    letter: Letter,
    next_jiggle_time: Instant,

    base: Base<Sprite2D>,
}
#[godot_api]
impl ISprite2D for ExtLetterSprite {
    fn init(base: Base<Self::Base>) -> Self {
        let letter = Letter::default();
        let next_jiggle_time = Self::get_next_jiggle_time();

        Self {
            letter,
            next_jiggle_time,

            base,
        }
    }

    fn ready(&mut self) {
        self.base_mut()
            .set_texture(load("res://assets/letters.png"));
        self.base_mut().set_hframes(26);

        let frame = self.letter as i32;
        self.base_mut().set_frame(frame);

        self.jiggle();
    }

    fn process(&mut self, _delta: f64) {
        if self.next_jiggle_time < Instant::now() {
            self.jiggle();

            self.next_jiggle_time = Self::get_next_jiggle_time();
        }
    }
}
impl ExtLetterSprite {
    fn jiggle(&mut self) {
        self.base_mut()
            .set_rotation_degrees(thread_rng().gen_range(-15.0..=15.0));
    }

    fn get_next_jiggle_time() -> Instant {
        Instant::now() + Duration::from_millis(thread_rng().gen_range(500..=2000))
    }
}
