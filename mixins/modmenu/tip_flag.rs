// The pause screen's mods icon, armed while the pause screen draws: the
// first tooltip of the frame draws it first, so the tooltip lies over it.
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

pub static ICON: Mutex<Option<[f32; 2]>> = Mutex::new(None);
pub static DRAWN: AtomicBool = AtomicBool::new(false);

pub fn arm(at: Option<[f32; 2]>) {
    if let Ok(mut i) = ICON.lock() {
        *i = at;
    }
}

/// The icon's button and its three lines.
pub fn icon(batch: &mut crate::hud::HudBatch, atlas: &crate::hud::HudAtlas, at: [f32; 2], cursor: (f32, f32)) {
    let [x, y] = at;
    let hover = cursor.0 >= x && cursor.0 < x + 20.0 && cursor.1 >= y && cursor.1 < y + 20.0;
    super::button(batch, atlas, x, y, 20.0, 20.0, &crate::text::Text::empty(), true, hover);
    for (i, width) in [12.0, 12.0, 8.0].into_iter().enumerate() {
        batch.fill(atlas, x + 4.0, y + 5.0 + i as f32 * 4.0, width, 2.0, 0xFFE8_E8E8);
    }
}

pub fn before_tooltip(batch: &mut crate::hud::HudBatch, atlas: &crate::hud::HudAtlas, cursor: (f32, f32)) {
    let at = ICON.lock().ok().and_then(|mut i| i.take());
    if let Some(at) = at {
        icon(batch, atlas, at, cursor);
        DRAWN.store(true, Ordering::Relaxed);
    }
}
