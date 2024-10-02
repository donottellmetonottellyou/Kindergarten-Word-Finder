pub mod letters;
pub mod managers;
pub mod words;

use godot::{init::EditorRunBehavior, prelude::*};

struct KindergartenWordFinderExtensionLibrary;
#[gdextension]
unsafe impl ExtensionLibrary for KindergartenWordFinderExtensionLibrary {
    fn editor_run_behavior() -> EditorRunBehavior {
        EditorRunBehavior::AllClasses
    }
}
