use crate::managers::letter::ExtLetterManager;

use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D, init)]
pub struct ExtWordManager {
    base: Base<Node2D>,
}
