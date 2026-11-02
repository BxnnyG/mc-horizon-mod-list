// The title screen's "Mods" button: where it is, whether the main screen
// is showing, and whether it was clicked (the game opens the list).
use std::sync::atomic::AtomicBool;

pub static OPEN: AtomicBool = AtomicBool::new(false);
pub static MAIN: AtomicBool = AtomicBool::new(false);

/// After the three icons between Realms and Options.
pub fn rect(w: f32, h: f32) -> [f32; 4] {
    let (w, h) = (w.ceil(), h.ceil());
    [(w / 2.0).floor() + 38.0, (h / 4.0).floor() + 48.0 + 72.0, 20.0, 20.0]
}
