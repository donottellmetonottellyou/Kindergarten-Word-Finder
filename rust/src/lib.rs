pub mod letters;
pub mod managers;
pub mod words;

use godot::prelude::*;

struct KindergartenWordFinderExtensionLibrary;
#[gdextension]
unsafe impl ExtensionLibrary for KindergartenWordFinderExtensionLibrary {}
