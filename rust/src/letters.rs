use godot::{
    engine::{ISprite2D, Sprite2D},
    obj::WithBaseField,
    prelude::*,
};
use rand::prelude::*;

use std::time::{Duration, Instant};

#[derive(Debug, Default, Clone, Copy, GodotConvert, Var, Export)]
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
    #[var(get, set = set_letter)]
    #[export]
    letter: Letter,
    #[var(get, set = set_jiggle)]
    #[export]
    jiggle: bool,

    next_jiggle_time: Instant,

    base: Base<Sprite2D>,
}
#[godot_api]
impl ISprite2D for ExtLetterSprite {
    fn init(base: Base<Self::Base>) -> Self {
        let letter = Letter::default();
        let jiggle = false;
        let next_jiggle_time = Instant::now();

        Self {
            letter,
            jiggle,

            next_jiggle_time,

            base,
        }
    }

    fn ready(&mut self) {
        self.set_letter(self.letter);
        self.set_jiggle(self.jiggle);
    }

    fn process(&mut self, _delta: f64) {
        self.jiggle_if_time()
    }
}
#[godot_api]
impl ExtLetterSprite {
    #[func]
    fn set_letter(&mut self, letter: Letter) {
        self.base_mut()
            .set_texture(load("res://assets/letters.png"));
        self.base_mut().set_hframes(26);
        self.base_mut().set_frame(letter as i32);
        self.letter = letter;
    }

    #[func]
    fn set_jiggle(&mut self, jiggle: bool) {
        if !jiggle {
            self.base_mut().set_rotation(0.0);
        }

        self.next_jiggle_time = Instant::now();
        self.jiggle = jiggle;
    }

    #[inline]
    fn jiggle_if_time(&mut self) {
        if !self.jiggle || self.next_jiggle_time > Instant::now() {
            return;
        }

        self.base_mut()
            .set_rotation_degrees(thread_rng().gen_range(-15.0..=15.0));
        self.next_jiggle_time += Duration::from_millis(thread_rng().gen_range(500..=2000));
    }
}
