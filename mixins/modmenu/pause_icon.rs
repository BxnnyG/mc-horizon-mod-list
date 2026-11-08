// The pause screen's mods icon goes in before the row's tooltip.
fn modmenu_pause_icon(batch: &mut HudBatch, atlas: &HudAtlas, cursor: (f32, f32)) -> core::ops::ControlFlow<()> {
    modmenu_tip::before_tooltip(batch, atlas, cursor);
    core::ops::ControlFlow::Continue(())
}
