// Under the mod list, the pause screen isn't drawn. Otherwise its tooltip
// draws the mods icon first (`pause_icon.rs`).
fn modmenu_pause() -> core::ops::ControlFlow<()> {
    if crate::game::modmenu::is_open() {
        core::ops::ControlFlow::Break(())
    } else {
        horizon_client::ui::modmenu_tip::arm(Some(crate::game::modmenu::pause_icon()));
        core::ops::ControlFlow::Continue(())
    }
}
