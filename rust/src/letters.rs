use godot::{
    engine::{ISprite2D, Sprite2D, TextureButton},
    prelude::*,
};
use rand::prelude::*;

use std::time::{Duration, Instant};

/// Enum that represents all the letters that can be used to create English
/// words in this game. `Letter`'s have a 0-indexed u8 `repr`, which can be
/// used for both the texture index of the `ExtLetter` sprite and as an ascii
/// byte (if you add `b'a'`).
#[derive(Debug, Default, Clone, Copy, GodotConvert, Var, Export)]
#[repr(u8)]
#[godot(via = u8)]
pub enum Letter {
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

/// Letter sprite, which also has an optional ability to be selected for
/// dynamic gameplay. Supports setting whether it jiggles and its letter
/// in the Godot editor UI. This uses a Sprite2D because it allows for
/// animation frames, which is easier than using an atlas texture.
#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct ExtLetter {
    base: Base<Sprite2D>,

    #[var(get, set = set_letter)]
    #[export]
    letter: Letter,
    #[var(get, set = set_jiggle)]
    #[export]
    jiggle: bool,

    button: Gd<TextureButton>,

    next_jiggle_time: Instant,
}
#[godot_api]
impl ISprite2D for ExtLetter {
    fn init(base: Base<Self::Base>) -> Self {
        let letter = Letter::default();
        let jiggle = false;
        let next_jiggle_time = Instant::now();

        let mut button = TextureButton::new_alloc();
        button.set_position(Vector2 { x: -8.0, y: -8.0 });
        button.set_size(Vector2 { x: 16.0, y: 16.0 });

        Self {
            letter,
            jiggle,

            next_jiggle_time,

            button,
            base,
        }
    }

    fn ready(&mut self) {
        self.set_letter(self.letter);
        self.set_jiggle(self.jiggle);

        let button = self.button.clone().upcast();
        self.base_mut().add_child(button);
    }

    fn process(&mut self, _delta: f64) {
        self.jiggle_if_time()
    }
}
#[godot_api]
impl ExtLetter {
    /// Set the letter. This will determine its sprite.
    #[func]
    pub fn set_letter(&mut self, letter: Letter) {
        self.base_mut()
            .set_texture(load("res://assets/letters.png"));
        self.base_mut().set_hframes(26);
        self.base_mut().set_frame(letter as i32);
        self.letter = letter;
    }

    /// Whether we should jiggle the letter periodically.
    #[func]
    pub fn set_jiggle(&mut self, jiggle: bool) {
        if !jiggle {
            self.base_mut().set_rotation(0.0);
        }

        self.next_jiggle_time = Instant::now();
        self.jiggle = jiggle;
    }

    /// Helper to connect the "pressed" functionality of the button to an
    /// arbitrary `Callable`.
    #[func]
    pub fn connect_button_pressed(&mut self, callable: Callable) {
        self.button.connect("pressed".into(), callable);
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
